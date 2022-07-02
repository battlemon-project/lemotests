use quote::quote;
use syn::{FnArg, Pat, PatIdent};
use lemotests::TxKind;
use proc_macro2::TokenStream;

pub(crate) struct FunctionBlueprint {
    pub(crate) contract_function_name: String,
    pub(crate) trait_method_name: String,
    pub(crate) args: Vec<FnArg>,
    pub(crate) account: Option<String>,
    pub(crate) tx_kind: TxKind,
    pub(crate) contract_name: String,
}

impl FunctionBlueprint {
    pub(crate) fn new(
        contract_function_name: String,
        trait_method_name: String,
        args: Vec<FnArg>,
        account: Option<String>,
        tx_kind: TxKind,
        contract_name: String,
    ) -> Self {
        Self {
            contract_function_name: contract_function_name.clone(),
            trait_method_name,
            args,
            account,
            tx_kind,
            contract_name,
        }
    }

    pub fn tx_kind(&self) -> &TxKind {
        &self.tx_kind
    }

    pub fn args_without_types(&self) -> Vec<&PatIdent> {
        self.args
            .iter()
            .map(|arg| {
                let FnArg::Typed(pat) = arg else { unreachable!() };
                let Pat::Ident(ident) = &*pat.pat else { unreachable!() };
                ident
            })
            .collect()
    }

    pub fn args_tt(&self) -> TokenStream {
        let args = self.args.clone();
        quote!(#(#args),*)
    }
}
