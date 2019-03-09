// imports
use rusoto_core::region::Region;
use rusoto_s3::S3Client;
use std::env;

// get s3 client function
pub fn establish_s3_connection() -> S3Client {
    // define the custom s3 region
    let custom_region = Region::Custom {
        name: "custom-iodione".to_string(),
        endpoint: env::var("iodine_s3_url").expect("Environment variable iodine_s3_url not set"),
    };
    // create the s3 client
    S3Client::new(custom_region)
}
