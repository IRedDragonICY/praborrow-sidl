use praborrow_sidl::{Diplomat, include_sidl};

// The include_sidl macro generates:
// 1. struct User { ... } with #[derive(Diplomat)]
// 2. trait UserService { ... }
include_sidl!("tests/example.sidl");

struct MyUserService;

#[async_trait::async_trait]
impl UserService for MyUserService {
    async fn get_user(&self, id: u64) -> User {
        User {
            id,
            username: "admin".to_string(),
        }
    }
}

#[tokio::test]
async fn test_sidl_generation() {
    let service = MyUserService;
    let user = service.get_user(1).await;
    assert_eq!(user.id, 1);
    assert_eq!(user.username, "admin");
}

#[test]
fn test_diplomat_trait() {
    // Verify User implements Diplomat (marker trait)
    fn assert_diplomat<T: praborrow_diplomacy::Diplomat>() {}
    assert_diplomat::<User>();
}
