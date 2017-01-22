error_chain!{
    foreign_links {
        PathPrefix(::std::path::StripPrefixError);
        Io(::std::io::Error);
        Posix(::nix::Error);
        Int(::std::num::ParseIntError);
    }
}
