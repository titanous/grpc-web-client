fn main() {
    tonic_build::configure()
        .build_client(false)
        .compile(
            &["../proto/helloworld.proto", "../proto/echo.proto"],
            &["../proto"],
        )
        .unwrap();
}
