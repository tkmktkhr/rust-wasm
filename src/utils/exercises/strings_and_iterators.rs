fn prefix_matches(prefix: &str, request_path: &str) -> bool {
  let mut split_request_path = request_path.split("/");
  for prefix_segment in prefix.split('/') {
    let Some(request_segment) = split_request_path.next() else {
        return false;
    };
    if request_segment != prefix_segment && prefix_segment != "*" {
      return false;
    }
  }
  true
}

#[test]
fn test_matches_without_wildcard() {
  assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
  assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
  assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

  assert!(!prefix_matches("/v1/publishers", "/v1"));
  assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
  assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
  assert!(prefix_matches(
    "/v1/publishers/*/books",
    "/v1/publishers/foo/books"
  ));
  assert!(prefix_matches(
    "/v1/publishers/*/books",
    "/v1/publishers/bar/books"
  ));
  assert!(prefix_matches(
    "/v1/publishers/*/books",
    "/v1/publishers/foo/books/book1"
  ));

  assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
  assert!(!prefix_matches(
    "/v1/publishers/*/books",
    "/v1/publishers/foo/booksByAuthor"
  ));
}
