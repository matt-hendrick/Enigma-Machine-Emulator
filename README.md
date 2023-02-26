# Enigma Machine Emulator

This is an Enigma machine emulator built in Rust that can be run from the command line. This implementation includes the 5 rotors used by the Enigma I variant and the Enigma M3 Army variant.

## Default Configuration

If no configuration settings are provided, this implementation defaults to using the following configuration:

| Setting Type    | Setting Value                    |
| --------------- | -------------------------------- |
| Right Rotor     | III (implemented as 3)           |
| Middle Rotor    | II (implemented as 2)            |
| Left Rotor      | I (implemented as 1)             |
| Rotor Positions | AAA (implemented as 000)         |
| Ring Settings   | AAA (implemented as 000)         |
| Reflector       | B                                |
| Plugboard       | No Default Plugboard Connections |

## How to Use

The following command line arguments can be used to alter the configuration of the emulated Enigma machine:

| Short | Long             | Description                                                                                                                                                                                                                               |
| ----- | ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| -o    | --original-text  | original text to encode/decode                                                                                                                                                                                                            |
| -t    | --rotor-types    | rotor type (three character long string. Valid rotor types are '1', '2', and '3') [default: 123]                                                                                                                                          |
| -p    | --rotor-position | rotor position (three character long string. Valid rotor positions are numbers between 0 and 25) [default: 000]                                                                                                                           |
| -s    | --ring-setting   | ring setting (three character long string. Valid ring settings are numbers between 0 and 25) [default: 000]                                                                                                                               |
| -r    | --reflector      | reflector (Valid reflector types are 'B' and 'C') [default: B]                                                                                                                                                                            |
| -b    | --plugboard      | plugboard (string of chars that will be used to create a hashmap in which each even indexed alphabetic char is mapped to the next odd indexed alphabetic char. Example 'AB CD' will swap 'A' <=> 'B' and swap 'C' <=> 'D' [default: none] |
| -h    | --help           | Print help                                                                                                                                                                                                                                |

## Examples

`enigma.exe -o test` encodes the text as `ZFDU` using the default configuration.

`enigma.exe -o test -t 543 -p 1,2,3 -s 4,5,6 -r c -b "AB CD EF GH"` encodes the text as `HFJL` with the settings being left rotor (III, rotor position 3 = 'D', ring setting 6 = 'G'), middle rotor (IV, rotor position 2 = 'C', ring setting 5 = 'F'), right rotor (V, rotor position 1 = 'B', ring setting 4 = 'E'), reflector 'C', and plugboard mapping of 'AB CD EF GH'

See the unit tests in main.rs for additional examples of encrypting and decrypting different inputs.

## Helpful resources on the mechanics of the Enigma Machine

- A [Computerphile video on building (and cracking) an Enigma machine in Java](https://www.youtube.com/watch?v=RzWB5jL5RX0). See that Java implementation [here](https://github.com/mikepound/enigma)

- A [3d walkthrough of the Enigma machine's mechanics](https://www.youtube.com/watch?v=ybkkiGtJmkM) by Jared Owen

- A [Numberphile video explaining the Enigma machine (demonstrated with an actual Enigma machine)](https://www.youtube.com/watch?v=G2_Q9FoD-oQ)

- The [Wikipedia page on Enigma rotors (and the variants which used each rotor type)](https://en.wikipedia.org/wiki/Enigma_rotor_details)

#### In-Browser Enigma Machine Emulators

The following in-browser Enigma emulators were useful for debugging and verifying that my implementation was encoding/decoding text correctly in all configurations and with varied inputs.

- [cryptii.com's Enigma emulator](https://cryptii.com/pipes/enigma-machine)

- [dcode.fr's Enigma emulator](https://www.dcode.fr/enigma-machine-cipher)

- [people.physik.hu-berlin.de's Enigma emulator](https://people.physik.hu-berlin.de/~palloks/js/enigma/enigma-u_v20_en.html)
