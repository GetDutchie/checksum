#[cfg(test)]
mod tests {
    use checksum::digest::compute_digest;

    #[test]
    fn compute_json_digest_tests() {
        let a = compute_digest(
            r#"
            {"a": 1, "b": 2}
        "#,
            Some("json"),
        )
        .unwrap();
        let b = compute_digest(
            r#"
            {"b": 2, "a": 1}
            "#,
            Some("json"),
        )
        .unwrap();
        assert_eq!(a, b);
        let a = compute_digest(
            r#"
            [
                {"foo": 1, "bar": 2},
                {"baz": 3, "bop": 4},
                {"zip": 5, "zap": 6}
            ]
        "#,
            Some("json"),
        )
        .unwrap();
        let b = compute_digest(
            r#"
            [
                {"baz": 3, "bop": 4},
                {"zip": 5, "zap": 6},
                {"foo": 1, "bar": 2}
            ]
            "#,
            Some("json"),
        )
        .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn compute_yaml_digest_tests() {
        let a = compute_digest(
            "apiVersion: v1\nkind: Pod\nmetadata:\n  name: my-pod\nspec:\n  containers:\n  - name: my-container\n    image: nginx\n"
        , Some("yaml")).unwrap();
        let b = compute_digest(
            "spec:\n  containers:\n  - name: my-container\n    image: nginx\nkind: Pod\napiVersion: v1\nmetadata:\n  name: my-pod\n", Some("yaml")
        ).unwrap();
        assert_eq!(a, b);
        let a = compute_digest(
            r#"
apiVersion: v1
kind: Pod
metadata:
  name: example-pod
  labels:
    app: example
spec:
  containers:
    - name: example-container
      image: nginx:latest
      command: |
        echo 'Starting...'
        echo 'Hello, Kubernetes!'
        echo 'Pod initialization complete.'
"#,
            Some("yaml"),
        )
        .unwrap();
        let b = compute_digest(
            r#"
kind: Pod
apiVersion: v1
spec:
  containers:
    - name: example-container
      command: |
        echo 'Starting...'
        echo 'Hello, Kubernetes!'
        echo 'Pod initialization complete.'
      image: nginx:latest
metadata:
  name: example-pod
  labels:
    app: example
"#,
            Some("yaml"),
        )
        .unwrap();
        assert_eq!(a, b);
    }
}
