// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(value) = optional_target {
            let word = value;  // 将 Some 的值赋给 word
            assert_eq!(word, target);  // 断言 word 和 target 相等
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(integer)=optional_integers.pop() {
            if integer.is_some()
            {assert_eq!(integer.unwrap(), cursor);
            cursor -= 1;}
        } 

        assert_eq!(cursor, 0);
    }
}
