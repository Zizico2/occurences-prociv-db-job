pub mod foundation {
    pub mod v1 {
        tonic::include_proto!("foundation.v1");
    }
}
// pub mod portugal_reimagined {
//     pub mod occurrences_service {
//         pub mod v1 {
//             tonic::include_proto!("portugal_reimagined.occurrences_service.v1");
//         }
//     }
// }
pub mod spatial_planning {
    pub mod v1 {
        tonic::include_proto!("spatial_planning.v1");
    }
}

pub mod occurrence {
    pub mod v1 {
        tonic::include_proto!("occurrence.v1");
    }
    pub mod c1 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c1.v1");
        }
    }
    pub mod c2 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c2.v1");
        }
    }
    pub mod c3 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c3.v1");
        }
    }
    pub mod c4 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c4.v1");
        }
    }

    pub mod c9 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c9.v1");
        }
    }
}

pub mod convert;
