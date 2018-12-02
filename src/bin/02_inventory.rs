/*!
# Day 2: Inventory Management System

You stop falling through time, catch your breath, and check the screen on the
device. "Destination reached. Current Year: 1518. Current Location: North Pole
Utility Closet 83N10." You made it! Now, to find those anomalies.

Outside the utility closet, you hear footsteps and a voice. "...I'm not sure
either. But now that so many people have chimneys, maybe he could sneak in that
way?" Another voice responds, "Actually, we've been working on a new kind of
suit that would let him fit through tight spaces like that. But, I heard that
a few days ago, they lost the prototype fabric, the design plans, everything!
Nobody on the team can even seem to remember important details of the project!"

"Wouldn't they have had enough fabric to fill several boxes in the warehouse?
They'd be stored together, so the box IDs should be similar. Too bad it would
take forever to search the warehouse for two similar box IDs..." They walk too
far away to hear any more.

Late at night, you sneak to the warehouse - who knows what kinds of paradoxes
you could cause if you were discovered - and use your fancy wrist device to
quickly scan every box and produce a list of the likely candidates (your puzzle
input).

To make sure you didn't miss any, you scan the likely candidate boxes again,
counting the number that have an ID containing exactly two of any letter and
then separately counting those with exactly three of any letter. You can
multiply those two counts together to get a rudimentary checksum and compare it
to what your device predicts.

For example, if you see the following box IDs:

- `abcdef` contains no letters that appear exactly two or three times.
- `bababc` contains two a and three b, so it counts for both.
- `abbcde` contains two b, but no letter appears exactly three times.
- `abcccd` contains three c, but no letter appears exactly two times.
- `aabcdd` contains two a and two d, but it only counts once.
- `abcdee` contains two e.
- `ababab` contains three a and three b, but it only counts once.

Of these box IDs, four of them contain a letter which appears exactly twice, and
three of them contain a letter which appears exactly three times. Multiplying
these together produces a checksum of 4 * 3 = 12.

What is the checksum for your list of box IDs?

## Part Two

Confident that your list of box IDs is complete, you're ready to find the boxes
full of prototype fabric.

The boxes will have IDs which differ by exactly one character at the same
position in both strings. For example, given the following box IDs:

    abcde
    fghij
    klmno
    pqrst
    fguij
    axcye
    wvxyz

The IDs `abcde` and `axcye` are close, but they differ by two characters (the
second and fourth). However, the IDs `fghij` and `fguij` differ by exactly one
character, the third (`h` and `u`). Those must be the correct boxes.

What letters are common between the two correct box IDs? (In the example above,
this is found by removing the differing character from either ID, producing
`fgij`.)
*/

use aoc18;

fn main() {
    let mut input = aoc18::ProgramInput::new(PUZZLE, INPUT);
    println!("Day 2: {}", PUZZLE);

    let lines = input.to_str().lines();
    println!(":: Answer 1 is {}", checksum(lines.clone()));

    /*
     * For part two there are several different methods that we could use.
     * Given hash length k and the number of hashes n:
     *
     * 1. For each position i in a hash, remove the ith column and add it
     *    to a hash map. This results in O(kkn) runtime and O(n) space.
     * 2. For each hash, compare a hamming distance to every other
     *    element. This results in O(knn) operations and O(1) space.
     */
    println!(
        ":: Answer 2 is {}",
        match extract_matching(lines) {
            Some(hash) => hash,
            None => "invalid".to_owned(),
        }
    );
}

fn extract_matching<'a>(hashes: impl IntoIterator<Item = &'a str>) -> Option<String> {
    let collection: Vec<_> = hashes.into_iter().collect();
    let n = collection.len();
    for i in 0..n {
        let x = collection[i];
        for j in (i + 1)..n {
            let y = collection[j];
            match num_differing(x, y) {
                0 => panic!("unexpected identical hashes"),
                1 => return Some(remove_differing(x, y)),
                _ => (),
            }
        }
    }
    None
}

fn remove_differing(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(x, y)| if x == y { Some(x) } else { None })
        .collect()
}

fn num_differing(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .map(|(x, y)| if x == y { 0 } else { 1 })
        .sum()
}

fn checksum<'a>(collection: impl IntoIterator<Item = &'a str>) -> u64 {
    // Tally number of identical characters in collection:
    let (two, three) = collection.into_iter().fold((0, 0), |(a, b), z| {
        let (x, y) = has_two_three(z);
        (a + x as u64, b + y as u64)
    });

    // Compute checksum:
    two * three
}

fn has_two_three(hash: &str) -> (bool, bool) {
    const A_ORDINAL: u8 = b'a';
    const Z_ORDINAL: u8 = b'z';
    const AZ_RANGE: usize = 1 + (Z_ORDINAL - A_ORDINAL) as usize;

    // Count occurence of each [a-z] character:
    let mut counts = [0; AZ_RANGE];
    for b in hash.bytes() {
        assert!(A_ORDINAL <= b && b <= Z_ORDINAL);
        let i = (b - A_ORDINAL) as usize;
        counts[i] += 1;
    }

    // Return whether there are any doubles or triples:
    let mut two = false;
    let mut three = false;
    for c in &counts {
        match c {
            2 => two = true,
            3 => three = true,
            _ => (),
        }
    }
    (two, three)
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    struct Test {
        input: &'static str,
        output: (bool, bool),
    }

    lazy_static! {
        static ref TESTS: Vec<Test> = vec![
            Test {
                input: "abcdef",
                output: (false, false),
            },
            Test {
                input: "bababc",
                output: (true, true),
            },
            Test {
                input: "abbcde",
                output: (true, false),
            },
            Test {
                input: "abcccd",
                output: (false, true),
            },
            Test {
                input: "aabcdd",
                output: (true, false),
            },
            Test {
                input: "abcdee",
                output: (true, false),
            },
            Test {
                input: "ababab",
                output: (false, true),
            },
        ];
    }

    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(TESTS.iter().map(|x| x.input)), 12);
    }

    #[test]
    fn test_has_two_three() {
        for t in TESTS.iter() {
            assert_eq!(has_two_three(t.input), t.output);
        }
    }

}

const PUZZLE: &'static str = "Inventory Management System";
const INPUT: &'static str = r"
uqcipadzntnheslgvjjozmkfyr
uqcipadzwtnhexlzvxjobmkfkr
cqcipadpwtnheslgyxjobmkfyr
ubnipadzwtnheslgvxjobmkfyw
uqcisadzwtnheslgvxjfbmkfor
uqcisaezwtnheslgvxkobmkfyr
uqcguadzwtnheslgvxjobmkfir
uqcipadzmtnhesldvxdobmkfyr
uqcipadzwtzheslgdxjtbmkfyr
uquipadzwtcheslgvxjobmkfbr
uqctpadzwtnhesjbvxjobmkfyr
ueciparzwtnheslgvxjobmkfyx
uqcipadzwtnhessgvxjkbmkfkr
uqcipamzwtnheslgvxioamkfyr
uciizadzwtnheslgvxjobmkfyr
uqcieadzwtnhesfgvxeobmkfyr
fqcipadzwtnreslgvkjobmkfyr
uqcipadzrtnherlgvxjobmklyr
uqcypadzwtnheslgvxjobmkxfr
uqcipadzwtnhemlgvxjobmvfur
uwciuadzwwnheslgvxjobmkfyr
uqcipadzwtnhcscgvxjobmkuyr
upripadzwtnheslovxjobmkfyr
uqcipadzltnheslgvxjobmkftc
uqcipadzwtnheslgvgjobmifsr
uqoipadzwtnheslgvxjosmkfkr
uqcipadzwtbhesrqvxjobmkfyr
uqcipadzwtnheslpvxjobmhfyx
uhcinadzwtnheslgvxjybmkfyr
uqcipadzwtnhhslgvxjabmkbyr
uecipadzwtnheslgvxjobqyfyr
uqcipadfwtnheslwvxjobgkfyr
uqcipadzvtnheshgvxzobmkfyr
fqcipadzwtcheslgvxjobmkfyt
uecipadzwtnheslgpxjbbmkfyr
uqclpadzwtnheslgvnjobukfyr
qqciprdzetnheslgvxjobmkfyr
uqcipahpwtnheslgvxjtbmkfyr
uqcidadzwtnhesljvxyobmkfyr
uqciradswtnqeslgvxjobmkfyr
uqcipadzwtrhmslgvxjobmkfyf
urcipadzjtnheslgvxfobmkfyr
uqcipadzwznheslgvxjobmkfcv
uqcipadowtnheslgyxjobmkfym
uqcigadzwtnheslgvxjoomkmyr
uqjipafzwtnheslgvejobmkfyr
uqcioadzwtnhhslgvxzobmkfyr
uqcgpadkwtnheslgvxjobhkfyr
ufciiadewtnheslgvxjobmkfyr
uqoipadzwtnheslgvxjllmkfyr
uqcipadzutnheslgwxxobmkfyr
uqcipadzwtlheslgaxjobmkfwr
uqcbpadzutnheslgvxjbbmkfyr
uucipadzwvnhesngvxjobmkfyr
uqcifadzwtnceslgvxjoumkfyr
ujcipadzwteheslgvxjobmkfyj
uqcipadzwtnheslqvxjobmkuyp
uqcipadzwtnheslgvxjoxmkxyw
uqcipaduwtnheslgvujmbmkfyr
uicipadnwtnheslgvxjobmbfyr
uqcipadzwteheslgvxjobbmfyr
uqcipadzwgnneslgvxjobmklyr
uqcipadzxtnhwslgvbjobmkfyr
uqcipaxwwtnheslxvxjobmkfyr
uocipadzwtnheslgvxjobqdfyr
uqciaauzwtnheslgtxjobmkfyr
uncipagzwtnkeslgvxjobmkfyr
uqcipadzwtnhehlgvxjohdkfyr
uqcipadzwtnheslgvxjobmspyz
uccipadzwtnhvsltvxjobmkfyr
uacipagzwtnheslgvxjoqmkfyr
tqcipaduwtnheslgvxjobmmfyr
uqcipadzwtnheslgvxqebmifyr
uecipadthtnheslgvxjobmkfyr
uocipadzwtnhdslgvkjobmkfyr
uqcipadtwtnheslgvxhobmufyr
uqkipadzwtnleslgtxjobmkfyr
uqcipadzjunheslgvxjobmnfyr
ubcipadzwtvheslgvxjobmkfyf
uqcipadzwpfheslgvxjsbmkfyr
uocipadzwtndeslgvxjobmmfyr
uqcipadzwtnheslgtxjobhkfyq
uqcipadzwtrheslgvxjobmyfya
uqcipadzwtvheslgvxjolgkfyr
uqcipidzwtaheslgvxjobmkfxr
uyzixadzwtnheslgvxjobmkfyr
uqyihadzwtnhedlgvxjobmkfyr
uqcipadzwtnhesltvejobqkfyr
uqciptdzwtnheslgyxlobmkfyr
uqcipzdzwtnhzslgvxjosmkfyr
uqcipadzwtnbeslgexjobmkfvr
uqcipadzwtnheslcwxjobmkkyr
uqcapadzwcnheslgvxjolmkfyr
uqcjpadzwtnhejlgvxjxbmkfyr
uqcipadwwtxweslgvxjobmkfyr
uqmipadzwtnhezlgvxjobmkyyr
uqcipubzwtnpeslgvxjobmkfyr
uecvpadzwtnheslgvxjocmkfyr
uqcipadzwfnheslgvxjibmkdyr
uqcipadzwtnheslgvxvfbykfyr
uqcipadzwtnheslgvgjoimkfyt
dqcqpaqzwtnheslgvxjobmkfyr
uqcipbdzwtnheslgvxjobmkghr
jqcipadzwtnheslgvxjgbmkzyr
uqcipadzwtnheslgvxqkqmkfyr
uqcipadzptnheslgvxjxbokfyr
uucijadzwtwheslgvxjobmkfyr
uccfpadzwtnheslgvxjobpkfyr
uqcipadzwtnheslgvxjobakeyq
uqcipadzwtnheolgvxqobjkfyr
imiipadzwtnheslgvxjobmkfyr
uqcehadzwtnheslgvxjobmkuyr
uqcipadzztnheslgvxjorokfyr
rqcixadzwtnheelgvxjobmkfyr
uqcipadzwtzheslgvxjodmkfyi
uqcipaezwtnwuslgvxjobmkfyr
uqcipadzwtnheslggxjobjkfyq
uqcipadzwkghesagvxjobmkfyr
uqcypqdzwtnheslgvxjobakfyr
iqcipadzwtnhezltvxjobmkfyr
uxcimadzwtnheslgvxjobmxfyr
uqcipaizwtvhwslgvxjobmkfyr
uqcipafzwtnheslgvxjpbmkfym
uqcipadzwinheslgvxlobmpfyr
uqcupadzwtnheslkvxmobmkfyr
uqcapadzwtnhesrgvxjobmkfsr
urcipafzwtnheslgvxjobmkfur
uqcipaczwtnheslgvbjobmknyr
uqcizadzztgheslgvxjobmkfyr
uqcipfdzwtnhesxgvxjobmkfyw
uqcipbdzwtnhyslgvxjobmcfyr
uqcipadzwanhezlgvxjobmkfwr
uvcipadzwtnheslgvxjbkmkfyr
uqcipajzwtnseslgvxjobmkfyq
uqcipvdzwtnheslgvmlobmkfyr
uqcipadzdgnheslgmxjobmkfyr
uqcipddzwtnhestgvpjobmkfyr
umcipadzwtdheslgvxjzbmkfyr
uqciuwdzwtnheslgvxjobmkflr
uqcipadzwtnheslgsxabbmkfyr
uceipadzwtnheslgvxjobgkfyr
mqcipadzwtnhesrgvxjobmjfyr
aqcipadvwtnheslgvxjobmkryr
uqsipadzwtnofslgvxjobmkfyr
uqcixadzwtfheslgvxjzbmkfyr
uqcipadnwfnheslgvxjohmkfyr
uqcivadzwtnheslfvxjobmkfyz
uqciprdzwtnheslgvxjobmkjir
uqcipadhbtnheslgvxjoxmkfyr
fqcipadzwtnhesfgvxjobmkfye
uqoipqdzwtnheqlgvxjobmkfyr
uqcipadzwtnhesltvxmobmkzyr
uqcipadzwtnhebqgvsjobmkfyr
uqcipadzwtnheslglxjobmfbyr
gqcipadzwtgheslgvxjobwkfyr
uqcipadzwtnheslgfxjzbmlfyr
ujcnpadzwtnheslrvxjobmkfyr
ujcivadzwtnheglgvxjobmkfyr
uqcitadzwgnheslgvxjofmkfyr
uqcipahzatnhmslgvxjobmkfyr
uqzipaizwtnheslgvujobmkfyr
uqcipadzltnheylgvnjobmkfyr
uqcidadzwtnhwsljvxyobmkfyr
uqcipadzwtihetlgvxjobhkfyr
oqcipabzwtnheslgvfjobmkfyr
uqcipadzwtnveslgvxjobzkfzr
uqcipadzwtjheslgqxjobmlfyr
uqcnpadzztnheslgvxjobmkoyr
uqciuadzwonheslgvxjobmkfyz
tqcipadzwtnheslgvxaobmqfyr
uqcipadtwtnhqslgvxjobmkeyr
uqcipadzwbnheslgvajobmsfyr
ubcopadzwtnhgslgvxjobmkfyr
uqcipydzwtwheslgvxjobakfyr
cqbijadzwtnheslgvxjobmkfyr
uscipadowtnheslgvxjobmkfcr
uqcipadzwtgheslnvxjobskfyr
uqcipzdzwtnzeslgkxjobmkfyr
uqcipawzwtnhrslgbxjobmkfyr
uqcipadzatchyslgvxjobmkfyr
uqcipadzotnpeslgvxjobmjfyr
uqcipagzwtnheslgvxjobmvfyt
uqcipadzwhnheslgvxyobmkfyo
uqcipadzwtnheslgmqjobmkfyc
uqcupadzwgnheslgvcjobmkfyr
uqcipabzwbnheslgvxjobmkwyr
uqciiadzwtnheslgvxjobmkfmz
uqkipauzwtnheslgvxjjbmkfyr
uqcipidzetnheslgvxjobmkfyi
uqcipadzwtnheslgqxjokmkfmr
uqcipadzqtnhesllvxjobmkfyk
uqccpadzwtnheslgmxsobmkfyr
uqcipadzwteheslgvljfbmkfyr
uqcipadxwinheslgaxjobmkfyr
uqcipadzwtnheslhvxyobmkfjr
aqcipadzwnnheslgvxjqbmkfyr
uvcipadzwtnheszgvxjobmkfyg
uqcipahzmtnheslgvxjobmkfir
ukcipadzbtnheslgvxjobmkfyb
uqcipadzwtnhemlgvqjobmkfpr
uqcipadzwtnheslgvmeobmkfpr
uqciphdrwtnheslgvxjobmkfyw
uqcipadzwtnheslevxqobzkfyr
uqcipadzwknzeslgvxnobmkfyr
wqcipadzwjnheslgvxjobbkfyr
uqcipadzwtdheslgvmjobmkjyr
uqvipadzwtnhextgvxjobmkfyr
uqhipadzwtnheslwvxjzbmkfyr
uqcipadzwtnherlgsxjobmksyr
uqcipadzwtnhesqgvxjotmvfyr
udcipadzwtnhekwgvxjobmkfyr
uqcjprdzwtnheslgvxjobmkfpr
uqcipadzatnheclgvqjobmkfyr
uqcbpadzctnheslqvxjobmkfyr
uqcipadzqtnhesluvxjobrkfyr
uqcipadzwtnhcslgvxjoomwfyr
uqcppadzwxnheslgwxjobmkfyr
uqcipadcwtnheslrvxjdbmkfyr
ukcipadzwtnhhslgvxjobmkgyr
uqckpadzwtnheslgvxjokmkiyr
uqcspadzwtjheslgvxjobmkfjr
uqcipadpwtnhsslgvxjobmkfyu
uqcepadzwtnheilgvbjobmkfyr
jqcipadiwtnheslgvxjobmkjyr
uqcipadzrtnseslgqxjobmkfyr
sqmipadzwtnhewlgvxjobmkfyr
uqcieadzhtnheslgvgjobmkfyr
uqcipadzwkwhewlgvxjobmkfyr
uqcipadzwtzheslgvxjpbqkfyr
uzcipadzjtnheslgvxjobmlfyr
uqcipadzwtnheslnvxjobmkfee
uqciyanzwtnheslgvxjoimkfyr
uqcipadqwtnheswghxjobmkfyr
uycipadzwtnheslovxjobmofyr
uqcipadzwtnheslgvxcozmxfyr
uqmipadzwtnxezlgvxjobmkfyr
uqcipadzftnheslgvxjotmkffr
aqcipaizwtnhesagvxjobmkfyr
uqcipcdzwtnheslgoajobmkfyr
uqcypadgwtnhesbgvxjobmkfyr
uqcipcdzwtnheslgvxjebmkfyb
uhcvpadzwtnheslgvxjobzkfyr
uqcipadzwtnpesagvxmobmkfyr
uqcipadzwtnidslgvxjobmkfor
uqcipadkwtnhesigvxjzbmkfyr
uqcypadlwtnheslsvxjobmkfyr
qqcipadzwtnheswgvxjobmkoyr
uqcipadzwtnheslgvxjhbmmcyr
uqcipadzwtnhesogvxjormkfmr
uqcipadzwtnhetcgvxgobmkfyr
";
