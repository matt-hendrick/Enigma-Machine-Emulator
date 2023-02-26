mod charindex;
mod enigma;
mod plugboard;
mod reflector;
mod rotor;
use clap::Parser;
use enigma::Enigma;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// original text to encode/decode
    #[arg(short = 'o', long)]
    original_text: String,

    /// rotor type (three character long string. Valid rotor types are '1', '2', and '3')
    #[arg(short = 't', long, default_value_t = ("123".to_string()))]
    rotor_types: String,

    /// rotor position (three character long string. Valid rotor positions are numbers between 0 and 25)
    #[arg(short = 'p', long, default_value_t = ("000".to_string()))]
    rotor_position: String,

    /// ring setting (three character long string. Valid ring settings  are numbers between 0 and 25)
    #[arg(short = 's', long, default_value_t = ("000".to_string()))]
    ring_setting: String,

    /// reflector (Valid reflector types are 'B' and 'C')
    #[arg(short = 'r', long, default_value_t = ("B".to_string()))]
    reflector: String,

    /// plugboard (string of chars that will be used to create a hashmap in which each even indexed alphabetic char is mapped to the next odd indexed alphabetic char.
    /// Example 'AB CD' will swap 'A' <=> 'B' and swap 'C' <=> 'D'
    #[arg(short = 'b', long, default_value_t = ("".to_string()))]
    plugboard: String,
}

fn main() {
    let args = Args::parse();

    let parsed_rotor_types = parse_rotor_types(args.rotor_types);
    let parsed_rotor_positions = parse_rotor_positions(args.rotor_position);
    let parsed_ring_settings = parse_ring_settings(args.ring_setting);
    let parsed_reflector = parse_reflector(args.reflector);
    let parsed_plugboard_mapping = parse_plugboard_mapping(args.plugboard);

    let mut enigma: Enigma = Enigma::new(
        parsed_rotor_types,
        parsed_rotor_positions,
        parsed_ring_settings,
        &parsed_reflector,
        &parsed_plugboard_mapping,
    );

    let encrypted_text = enigma.encrypt_string(args.original_text.clone());

    println!(
        "Encrypted text = {}. Original Text = {}",
        encrypted_text, args.original_text
    );
}

fn parse_rotor_types(rotor_type_string: String) -> [u8; 3] {
    let mut output_array: [u8; 3] = [1; 3];
    let mut count = 0;
    for char in rotor_type_string.chars() {
        if count > 2 {
            panic!("Only 3 digits can be provided for rotor types.")
        }
        if !char.is_digit(10) {
            panic!("{} is not a digit. All rotor typess must be digits.", char)
        }
        let num = char.to_owned().to_digit(10).unwrap() as u8;
        if num < 1 || num > 5 {
            panic!("{} is not a valid rotor type.", char)
        } else {
            output_array[count] = num;
        }
        count += 1;
    }

    output_array
}

fn parse_rotor_positions(rotor_position_string: String) -> [u8; 3] {
    parse_rotor_positions_or_ring_settings(rotor_position_string, true)
}

fn parse_ring_settings(ring_setting_string: String) -> [u8; 3] {
    parse_rotor_positions_or_ring_settings(ring_setting_string, false)
}

fn parse_rotor_positions_or_ring_settings(
    string_to_parse: String,
    is_rotor_position: bool,
) -> [u8; 3] {
    let input_type = if is_rotor_position {
        "rotor position"
    } else {
        "ring setting"
    };
    let mut output_array: [u8; 3] = [0; 3];

    let split_string = string_to_parse.split(",");

    let mut count = 0;
    for split_section in split_string {
        if count > 2 {
            panic!("Only 3 digits can be provided for {}s.", input_type)
        }

        let num = split_section.parse::<u8>().unwrap();

        if num > 25 {
            panic!("{} is not a valid {}.", split_section, input_type)
        } else {
            output_array[count] = num;
        }

        count += 1;
    }

    output_array
}

fn parse_reflector(reflector_string: String) -> String {
    let output_string = reflector_string.to_ascii_uppercase();
    if output_string == "B" || output_string == "C" {
        output_string
    } else {
        panic!("{} is not a valid reflector type", output_string)
    }
}

fn parse_plugboard_mapping(plugboard_string: String) -> String {
    if plugboard_string.len() > 29 {
        panic!("Too many characters provided for plugboard mapping")
    }
    let original_string = plugboard_string.to_ascii_uppercase();
    let mut output_string: String = String::new();
    for char in original_string.chars() {
        if !char.is_ascii_uppercase() {
            continue;
        } else {
            output_string.push(char);
        }
    }
    output_string
}

#[cfg(test)]
mod tests {
    use crate::Enigma;
    #[test]
    fn encrypt_decrypt_abcd() {
        let mut enigma = Enigma::new([1, 2, 3], [0, 0, 0], [0, 0, 0], "B", "");
        let original_text = "ABCD";
        let encrypted_text = enigma.encrypt_string(original_text.to_string());
        assert_eq!(encrypted_text, "FUVE");

        enigma = Enigma::new([1, 2, 3], [0, 0, 0], [0, 0, 0], "B", "");
        let decrypted_text = enigma.encrypt_string(encrypted_text);
        assert_eq!(decrypted_text, "ABCD");
    }

    #[test]
    fn encrypt_decrypt_abcd_alt_config() {
        let mut enigma = Enigma::new([3, 1, 2], [19, 2, 14], [11, 22, 6], "C", "ABCDGLZUVFJP");
        let original_text = "ABCD";
        let encrypted_text = enigma.encrypt_string(original_text.to_string());
        assert_eq!(encrypted_text, "MHIF");

        enigma = Enigma::new([3, 1, 2], [19, 2, 14], [11, 22, 6], "C", "ABCDGLZUVFJP");
        let decrypted_text = enigma.encrypt_string(encrypted_text);
        assert_eq!(decrypted_text, "ABCD");
    }

    #[test]
    fn encrypt_decrypt_sentence() {
        let mut enigma = Enigma::new([1, 2, 3], [0, 0, 0], [0, 0, 0], "B", "");
        let original_text = "This is an Enigma machine emulator built in Rust that can be run from the command line.";
        let encrypted_text = enigma.encrypt_string(original_text.to_string());
        assert_eq!(
            encrypted_text,
            "ZPJJSVSCGWEBKDKJMXCDQRWLRDBCGJWPYMMHSKYRGLKLKCYFRIOVTEJLDAGPUYHSBUHNUQ"
        );

        enigma = Enigma::new([1, 2, 3], [0, 0, 0], [0, 0, 0], "B", "");
        let decrypted_text = enigma.encrypt_string(encrypted_text);
        assert_eq!(
            decrypted_text,
            "THISISANENIGMAMACHINEEMULATORBUILTINRUSTTHATCANBERUNFROMTHECOMMANDLINE"
        )
    }

    #[test]
    fn encrypt_decrypt_sentence_alt_config() {
        let mut enigma = Enigma::new([3, 1, 2], [19, 2, 14], [11, 22, 6], "C", "ABCDGLZUVFJP");
        let original_text = "This is an Enigma machine emulator built in Rust that can be run from the command line.";
        let encrypted_text = enigma.encrypt_string(original_text.to_string());
        assert_eq!(
            encrypted_text,
            "YBCUOBEYOPKMCCKMTNPAJVIKSSRYXIADAJCJEVHOHTMIVDRALDNFKTECHMHUSYRZLSIBZM"
        );

        enigma = Enigma::new([3, 1, 2], [19, 2, 14], [11, 22, 6], "C", "ABCDGLZUVFJP");
        let decrypted_text = enigma.encrypt_string(encrypted_text);
        assert_eq!(
            decrypted_text,
            "THISISANENIGMAMACHINEEMULATORBUILTINRUSTTHATCANBERUNFROMTHECOMMANDLINE"
        )
    }

    #[test]
    fn encrypt_decrypt_paragraph() {
        let mut enigma = Enigma::new([1, 2, 3], [0, 0, 0], [0, 0, 0], "B", "");
        let original_text = "The Enigma machine is a cipher device developed and used in the early- to mid-20th century to protect commercial, diplomatic, and military communication. It was employed extensively by Nazi Germany during World War II, in all branches of the German military. The Enigma machine was considered so secure that it was used to encipher the most top-secret messages. The Enigma has an electromechanical rotor mechanism that scrambles the 26 letters of the alphabet. In typical use, one person enters text on the Enigma's keyboard and another person writes down which of the 26 lights above the keyboard illuminated at each key press. If plain text is entered, the illuminated letters are the ciphertext. Entering ciphertext transforms it back into readable plaintext. The rotor mechanism changes the electrical connections between the keys and the lights with each keypress.";
        let encrypted_text = enigma.encrypt_string(original_text.to_string());
        assert_eq!(
            encrypted_text,
            "ZPTDBAMVIDJUROAXUEOGDWLPLACJQPAWJSTEYIDBIZWCRNBZUHRXAPIMEDKKLZSRXOOSNFQSMXJHOXMEZQBICEQKLXXXRCPJOENDYYEPCIVXWOJRVZDKLDSKLRJZKJMRJBPYGXDMBFMVUROXBGXDTKRGQOSVIPNVJZGPOTRYDNDBAVLSLUQQXCRSUCIXNVCVILIBWBHDPZVKVPXPEBASXQJNZXBBEUUQJZSRNDXUDCJJOVWXDTYBGFUWVHBJNKPQHXMPRPEDZVDBWFJFRNAPCQSQXJDKMQJMQZTJINWFOITXOHNYVZZUDVMJSTEOETCCXGRQEUMXWIDGVYTAOWGOOLJXTWFXTTWPGAIOLQCGEQIJYHKVLZLJCOOYXMRZXGGQVOQIWOMFLTGUXRHHJKRLZSDNILSCAAWKXZMDVNPFQXCACBYLBCETIBHCJELENTTXLLBPBBROTBSYTIJJDHRUOSCVJBCGFXQCCGXFYYCESITPOKYHSPXQVVIMXZGJGBTFKDFAQYGOWYOCIYZCCAFMKODIGRIGDEBFWWIZTOILUQOYCLVVYJJCFFUFPNDRYOFOPAAXLWFJGJYBAFEUZQESZVGUTWQXMNSHXYOKAUBGQCKPLFSJMOQQWGOQEIETKJQYCJRYNTKVIZNNMJHVAEVZYWRZBDCHGMNUQATLIMPFQJMWVMLCSWLMAJXZCXCEVRMIOZQKMW"
        );

        enigma = Enigma::new([1, 2, 3], [0, 0, 0], [0, 0, 0], "B", "");
        let decrypted_text = enigma.encrypt_string(encrypted_text);
        assert_eq!(decrypted_text, "THEENIGMAMACHINEISACIPHERDEVICEDEVELOPEDANDUSEDINTHEEARLYTOMIDTHCENTURYTOPROTECTCOMMERCIALDIPLOMATICANDMILITARYCOMMUNICATIONITWASEMPLOYEDEXTENSIVELYBYNAZIGERMANYDURINGWORLDWARIIINALLBRANCHESOFTHEGERMANMILITARYTHEENIGMAMACHINEWASCONSIDEREDSOSECURETHATITWASUSEDTOENCIPHERTHEMOSTTOPSECRETMESSAGESTHEENIGMAHASANELECTROMECHANICALROTORMECHANISMTHATSCRAMBLESTHELETTERSOFTHEALPHABETINTYPICALUSEONEPERSONENTERSTEXTONTHEENIGMASKEYBOARDANDANOTHERPERSONWRITESDOWNWHICHOFTHELIGHTSABOVETHEKEYBOARDILLUMINATEDATEACHKEYPRESSIFPLAINTEXTISENTEREDTHEILLUMINATEDLETTERSARETHECIPHERTEXTENTERINGCIPHERTEXTTRANSFORMSITBACKINTOREADABLEPLAINTEXTTHEROTORMECHANISMCHANGESTHEELECTRICALCONNECTIONSBETWEENTHEKEYSANDTHELIGHTSWITHEACHKEYPRESS");
    }

    #[test]
    fn encrypt_decrypt_paragraph_alt_config() {
        let mut enigma = Enigma::new([3, 1, 2], [19, 2, 14], [11, 22, 6], "C", "ABCDGLZUVFJP");
        let original_text = "The Enigma machine is a cipher device developed and used in the early- to mid-20th century to protect commercial, diplomatic, and military communication. It was employed extensively by Nazi Germany during World War II, in all branches of the German military. The Enigma machine was considered so secure that it was used to encipher the most top-secret messages. The Enigma has an electromechanical rotor mechanism that scrambles the 26 letters of the alphabet. In typical use, one person enters text on the Enigma's keyboard and another person writes down which of the 26 lights above the keyboard illuminated at each key press. If plain text is entered, the illuminated letters are the ciphertext. Entering ciphertext transforms it back into readable plaintext. The rotor mechanism changes the electrical connections between the keys and the lights with each keypress.";
        let encrypted_text = enigma.encrypt_string(original_text.to_string());
        assert_eq!(
            encrypted_text,
            "YBLCXXIGYAHRPPCRJCUKPWOTICFSGVTIGHVKSCDKUKJYAHMUIXWIHHDNRGZXAULJVHRHIQGWJXAIXLKDAMGSOQUVDNLYOBWFXNWNIJGJAXQHYPNHCTXSWLUTXAWTZXLCUXKLMPJCECJYKYLXKLXAPWACNTESUVKCHFBZFORRCKAWFFKRUDEVHNXMEGOYYZUYSOPYJFVMTTZTJUZMWUYHJDTFJFNHLUMPXHMIRUPJHYDFYMHSHAFXPXHLVNTDLEWFKQYGNQETJSAHWGVDJLBPXIUKLQGPXYAYDHCGOJAHXQHHJTXGXVFBWAFXFVATQVOWTFNWUHRLURRHGTTLEQHLHDERCMSZYFMNZXJOGMCWMDRXDYTTZSGCWKMEFBCCFNGEZUFWFKQHNTADYQLFAFHNIGWNGVJKZPAPRBIFCKBDKKPNKWERFFPLSEIHQGGXCMLVPNLJBBYUXJELFNMTCHBLXTZNSNJEKRADYVSVXRNNOLXYJKIKKQOEEFVQPLORNPGIMFHGXLOPQKPWGHNKWWFBMQOCFUEQIWTXBDBGPLIYZLDRYMTPGQWVQRZZCQCMSMDAWCXRBHVIBZRHYUQXRBPRJJEHXXLBGFQCEJPAUHWYGZSLCWMVZMPVBDLYZSLTSDFTZZQNQWQFQZZQBYDJDEPYXJBICWHLDBGAUZWETJRPUMEWUZPDIYRXHYTTSPSELIOBUSZULK"
        );

        enigma = Enigma::new([3, 1, 2], [19, 2, 14], [11, 22, 6], "C", "ABCDGLZUVFJP");
        let decrypted_text = enigma.encrypt_string(encrypted_text);
        assert_eq!(decrypted_text, "THEENIGMAMACHINEISACIPHERDEVICEDEVELOPEDANDUSEDINTHEEARLYTOMIDTHCENTURYTOPROTECTCOMMERCIALDIPLOMATICANDMILITARYCOMMUNICATIONITWASEMPLOYEDEXTENSIVELYBYNAZIGERMANYDURINGWORLDWARIIINALLBRANCHESOFTHEGERMANMILITARYTHEENIGMAMACHINEWASCONSIDEREDSOSECURETHATITWASUSEDTOENCIPHERTHEMOSTTOPSECRETMESSAGESTHEENIGMAHASANELECTROMECHANICALROTORMECHANISMTHATSCRAMBLESTHELETTERSOFTHEALPHABETINTYPICALUSEONEPERSONENTERSTEXTONTHEENIGMASKEYBOARDANDANOTHERPERSONWRITESDOWNWHICHOFTHELIGHTSABOVETHEKEYBOARDILLUMINATEDATEACHKEYPRESSIFPLAINTEXTISENTEREDTHEILLUMINATEDLETTERSARETHECIPHERTEXTENTERINGCIPHERTEXTTRANSFORMSITBACKINTOREADABLEPLAINTEXTTHEROTORMECHANISMCHANGESTHEELECTRICALCONNECTIONSBETWEENTHEKEYSANDTHELIGHTSWITHEACHKEYPRESS");
    }
}
