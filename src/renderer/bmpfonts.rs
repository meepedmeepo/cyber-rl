


///TODO: need to add testing so that if it is greater than 255 then this fucking complains! will probs save my ass down the line lmao
pub fn cp437_to_xy(codepoint : u8) -> (i32,i32)
{
    ((codepoint%16u8) as i32,(codepoint/16u8) as i32)
}



#[cfg(test)]
mod tests 
{
    use codepage_437::{ToCp437, CP437_WINGDINGS};

    use super::*;

    #[test]
    fn cp437()
    {
        let res1 = cp437_to_xy(0);
        let res2 = cp437_to_xy(15u8);
        let res3 = cp437_to_xy(16u8);
        let res4 = cp437_to_xy(*"@".to_cp437(&CP437_WINGDINGS).unwrap().first().unwrap());

        assert_eq!(res1,(0,0));
        assert_eq!(res2,(0,15));
        assert_eq!(res3, (1,0));
        assert_eq!(res4, (4,0))
    }

}