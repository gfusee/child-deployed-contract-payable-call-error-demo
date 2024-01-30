#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Empty: ContractBase {
    #[init]
    fn init(&self) {}

    #[endpoint(deployAnotherContract)]
    fn deploy_another_contract(
        &self,
        source_address: ManagedAddress<Self::Api>
    ) {
        self.send_raw()
            .deploy_from_source_contract(
                600_000_000,
                &BigUint::zero(),
                &source_address,
                CodeMetadata::PAYABLE_BY_SC,
                &ManagedArgBuffer::new()
            );
    }

    #[endpoint(forwardPayment)]
    #[payable("*")]
    fn forward_payment(&self) -> EsdtTokenPayment<Self::Api> {
        let payment = self.call_value().single_esdt();

        self.send()
            .direct_esdt(
                &self.blockchain().get_caller(),
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

        payment
    }

    #[endpoint(syncCallForwardPayment)]
    #[payable("*")]
    fn sync_call_forward_payment(
        &self,
        contract_address: ManagedAddress<Self::Api>
    ) -> EsdtTokenPayment<Self::Api> {
        self.contract_proxy(
            contract_address
        )
            .forward_payment()
            .with_esdt_transfer(self.call_value().single_esdt())
            .execute_on_dest_context()
    }

    #[proxy]
    fn contract_proxy(&self, address: ManagedAddress<Self::Api>) -> crate::Proxy<Self::Api>;
}
