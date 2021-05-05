fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["../proto/helloworld.proto", "../proto/echo.proto"],
            &["../proto"],
        )
        .unwrap();
}
