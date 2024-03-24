mod  modules;
use modules::{
    read_user_input,
    read_line,
    when_bytes_arent_utf_8,
};
fn main() {
    //read_user_input::read_user_input();
    //read_user_input::no_panicking_read_user_input();
    //read_user_input::trim_user_input();    

    // there are 3 take that will allow us to enter
    //read_user_input::take_user_input();
    //read_user_input::next_user_input();

    // this will run forever
    //read_user_input::infinite_user_input();

    //read_user_input::collect_with_limit_user_input();
    //read_user_input::parsing_input_into_numbers();

    //read_user_input::error_kind_user_input();

    //read_line::read_line_from_stdin();

    when_bytes_arent_utf_8::bytes_arent_utf_8();

}
