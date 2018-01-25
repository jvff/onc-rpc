#[macro_export]
macro_rules! onc_rpc {
    () => {};

    (
        program ( $module:ident :: $name:ident ) {
            id = $id:expr ;
            version = $version:expr ;

            $( #[$trait_attribute:meta] )*
            procedures $procedures:tt
        }

        $(
            program ( $rest_module:ident :: $rest_name:ident ) {
                id = $rest_id:expr ;
                version = $rest_version:expr ;

                $( export $rest_exports:tt )*

                $( #[$rest_trait_attribute:meta] )*
                procedures $rest_procedures:tt
            }
        )*
    ) => {
        onc_rpc!  {
            program ( $module::$name ) {
                id = $id;
                version = $version;

                export {}

                $( #[$trait_attribute] )*
                procedures $procedures
            }

            $(
                program ( $rest_module::$rest_name ) {
                    id = $rest_id;
                    version = $rest_version;

                    $( export $rest_exports )*

                    $( #[$rest_trait_attribute] )*
                    procedures $rest_procedures
                }
            )*
        }
    };

    (
        program ( $module:ident :: $name:ident ) {
            id = $id:expr ;
            version = $version:expr ;

            export $exports:tt

            $( #[$trait_attribute:meta] )*
            procedures $procedures:tt
        }

        $(
            program ( $rest_module:ident :: $rest_name:ident ) {
                id = $rest_id:expr ;
                version = $rest_version:expr ;

                $( export $rest_exports:tt )*

                $( #[$rest_trait_attribute:meta] )*
                procedures $rest_procedures:tt
            }
        )*
    ) => {
        onc_rpc_program! {
            $module,
            $name,
            $id,
            $version,
            $( #[$trait_attribute] )*
            $procedures,
            $exports,
        }

        pub use self::$module::$name;

        onc_rpc_program_export!($module $exports);

        onc_rpc! {
            $(
                program ( $rest_module::$rest_name ) {
                    id = $rest_id;
                    version = $rest_version;

                    $( export $rest_exports )*

                    $( #[$rest_trait_attribute] )*
                    procedures $rest_procedures
                }
            )*
        }
    };
}
