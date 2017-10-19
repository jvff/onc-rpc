#[macro_export]
macro_rules! onc_rpc_program_procedure_parameters {
    ( $(,)* ) => {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct Parameters;

        impl Default for Parameters {
            fn default() -> Self {
                Parameters
            }
        }

        onc_rpc_program_procedure_parameters_rpc_call!();
    };

    ( $name:ident : $type:ty $(,)* ) => {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct Parameters($type);

        impl From<$type> for Parameters {
            fn from($name: $type) -> Self {
                Parameters($name)
            }
        }

        impl From<Parameters> for $type {
            fn from(parameters: Parameters) -> Self {
                match parameters {
                    Parameters(value) => value,
                }
            }
        }

        onc_rpc_program_procedure_parameters_rpc_call!();
    };

    ( $( $name:ident : $type:ty ),* $(,)* ) => {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        pub struct Parameters {
            $( $name: $type, )*
        }

        impl Parameters {
            pub fn new($( $name: $type ),*) -> Self {
                Parameters {
                    $( $name, )*
                }
            }

            $(
                pub fn $name(&self) -> $type {
                    self.$name
                }
            )*
        }

        onc_rpc_program_procedure_parameters_rpc_call!();
    };
}

#[macro_export]
macro_rules! onc_rpc_program_procedure_parameters_rpc_call {
    () => {
        impl RpcCall for Parameters {
            type Procedure = Procedure;

            fn parameters(&self) -> Parameters {
                self.clone()
            }
        }
    }
}
