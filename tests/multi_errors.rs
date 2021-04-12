use async_graphql::*;

#[tokio::test]
pub async fn test_multi_errors() {
    struct MyObj;

    #[Object]
    impl MyObj {
        async fn value1(&self) -> i32 {
            10
        }

        async fn value2(&self) -> Result<i32, &str> {
            Err("error1")
        }

        async fn value3(&self) -> Result<i32, &str> {
            Err("error2")
        }
    }

    struct Query;

    #[Object]
    impl Query {
        async fn value1(&self) -> i32 {
            10
        }

        async fn value2(&self) -> Result<i32, &str> {
            Err("error1")
        }

        async fn value3(&self) -> Result<i32, &str> {
            Err("error2")
        }

        async fn obj(&self) -> MyObj {
            MyObj
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let resp = schema
        .execute("{ value1 value2 value3 obj { value1 value2 value3 } }")
        .await;
    assert_eq!(
        resp.data,
        value!({
            "value1": 10,
            "value2": null,
            "value3": null,
            "obj": {
                "value1": 10,
                "value2": null,
                "value3": null,
            }
        })
    );
    assert_eq!(resp.errors, vec![]);
}
