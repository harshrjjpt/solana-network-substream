// @generated
// @@protoc_insertion_point(attribute:acme)
pub mod acme {
    include!("acme.rs");
    // @@protoc_insertion_point(acme)
}

pub mod sf {
    pub mod solana {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.solana.type.v1)
           
        }
    }
    pub mod substreams {
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
           
        }
        pub mod sink {
            pub mod database {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.database.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.database.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.database.v1)
                }
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
       
    }
}
