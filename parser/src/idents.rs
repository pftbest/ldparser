use nom::alphanumeric;

named!(pub symbol<&str, &str>,
    call!(alphanumeric)
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_symbol() {
        //".text"
        //"*"
        //"hello*.o"
        //"\"spaces are ok, just quote the identifier\""
        //"this+is-another*crazy[example]"
    }
}
