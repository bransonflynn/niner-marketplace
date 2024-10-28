#[allow(dead_code)]
pub mod nm_account {
    use crate::nm_listing::nm_listing;
    use crate::{
        nm_common::{nm_date, nm_image, nm_keyword},
        nm_item::nm_item,
    };

    pub struct Account {
        id: u64,
        username: String,
        password: Password,
        bio: String,
        profile_picture: nm_image::Image,
        background_picture: nm_image::Image,
        reputation: Reputation,
        account_dates: AccountDates,
        listing_data: ListingData,
    }
    impl Account {
        fn old_enough(&self) -> bool {
            // get current m/d/y
            // check if acount_dates.user_birthday is >= 18 years old
            true
        }

        fn add_listing_offered(&mut self, listing: nm_listing::Listing) {
            self.listing_data.listings_offered.push(listing);
        }

        fn add_listing_featured(&mut self, listing: nm_listing::Listing) {
            self.listing_data.listings_featured.push(listing);
        }

        // reputation functions
        pub fn get_rep(&self) -> i32 {
            self.reputation.rep
        }
        pub fn set_rep(&mut self, value: i32) {
            self.reputation.rep = value
        }
        pub fn mod_rep(&mut self, amount: i32) {
            self.reputation.rep += amount
        }
    }

    pub fn create_account() {}

    struct Password {
        password: String,
    }
    impl Password {
        // primitive - needs proper handling
        pub fn is_correct(&self, _entry: String) -> bool {
            false
        }
    }

    pub struct Reputation {
        rep: i32,
        display: String,
    }
    impl Reputation {
        pub fn display_rep(&self) -> String {
            let rep: i32 = self.rep;    // extract value once so we don't have to access it
            String::from("temp_reputation_display_rep")
        }
    }

    pub struct AccountDates {
        user_birthday: nm_date::Date,
        account_birthday: nm_date::Date,
    }
    impl AccountDates {}

    pub struct ListingData {
        listings_offered: Vec<nm_listing::Listing>, // listings up for sale
        listings_featured: Vec<nm_listing::Listing>, // top x listings displayed on profile
        watched_listings: Vec<nm_listing::Listing>, // listings that the user wants, receive notifications for offers on listing
        wanted_items: Vec<nm_item::Item>,           // "looking for" items
        wanted_keywords: Vec<nm_keyword::Keyword>,  // preferred keywords
    }
    impl ListingData {}
}
