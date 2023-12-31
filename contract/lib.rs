#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Mintable)]
#[openbrush::contract]
pub mod y_psp22_token {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct YToken {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl YToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn total_supply_works() {
            let token = YToken::new(100);
            assert_eq!(PSP22Impl::total_supply(&token), 100);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod e2e_tests {
        use super::*;
        use ink::primitives::AccountId;
        use ink_e2e::subxt::tx::Signer;
        use ink_e2e::subxt::utils::AccountId32;
        use ink_e2e::{build_message, Keypair, PolkadotConfig};
        use openbrush::contracts::psp22::psp22_external::PSP22;

        type ContractRef = YTokenRef;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        fn address_of(keypair: &Keypair) -> AccountId {
            let address: AccountId32 = <Keypair as Signer<PolkadotConfig>>::account_id(keypair);
            address.0.into()
        }

        async fn balance_of(
            client: &mut ink_e2e::Client<PolkadotConfig, ink_env::DefaultEnvironment>,
            address: ink::primitives::AccountId,
            account: ink::primitives::AccountId,
        ) -> Balance {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.balance_of(account));
            let result = client
                .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                .await
                .return_value();
            result
        }

        #[ink_e2e::test]
        async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("y_psp22_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of(&ink_e2e::alice())));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("y_psp22_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of(&ink_e2e::bob()), 50, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice =
                balance_of(&mut client, address, address_of(&ink_e2e::alice())).await;

            let balance_of_bob =
                balance_of(&mut client, address, address_of(&ink_e2e::bob())).await;

            assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
            assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_above_the_amount(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("y_psp22_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of(&ink_e2e::bob()), 101, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(
                result.return_value(),
                Err(PSP22Error::InsufficientBalance)
            ));

            Ok(())
        }
    }
}
