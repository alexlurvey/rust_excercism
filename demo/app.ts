import { getDayOfYear } from 'date-fns/getDayOfYear';
import init, {
  after,
  anagram_for,
  annotate,
  bob,
  brackets_are_balanced,
  build_proverb,
  collatz,
  difference,
  factors,
  grains_square,
  grains_total,
  is_armstrong_number,
  is_leap_year,
  luhn,
  nth,
  plants,
  public_key,
  private_key,
  raindrops,
  reverse,
  secret,
  series,
  sing,
  sublist,
  sum_of_multiples,
  years_during,
  Clock,
  Comparison,
  Planet,
} from "../pkg/wasm_playground";

const toRustDate = (date: Date) => {
  return [
    date.getFullYear(),
    getDayOfYear(date),
    date.getHours(),
    date.getMinutes(),
    date.getSeconds(),
    date.getMilliseconds(),
  ]
}

init().then(() => {
  console.log('*************************************************')
  console.log('Excercise #1: Reverse')
  console.log('*************************************************')
  console.log('reversing - uüu', reverse('uüu'));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #2: Gigasecond')
  console.log('*************************************************')
  const date = new Date(2015, 0, 24, 22, 0);
  const rust_date = toRustDate(date);
  console.log('dates', date, rust_date)
  const giga = after(rust_date) as [number, number, number, number, number, number];
  console.log('after', new Date(...giga));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #3: Clock')
  console.log('*************************************************')
  const clock = new Clock(1, 30);
  console.log('clock start', clock.stringify());
  console.log('subtract 45 min', clock.add_minutes(-45).stringify());
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #4: Anagram')
  console.log('*************************************************')
  const anagrams = anagram_for("LISTEN", ["LISTEN", "Silent"]);
  console.log('anagrasm for: LISTEN', anagrams)
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #5: Space Age')
  console.log('*************************************************')
  const seconds = BigInt(1_000_000_000); // 31.69 earth years
  const mercury = years_during(BigInt(1000000000), Planet.Mercury);
  const venus = years_during(BigInt(1000000000), Planet.Venus);
  const earth = years_during(BigInt(1000000000), Planet.Earth);
  const mars = years_during(BigInt(1000000000), Planet.Mars);
  const jupiter = years_during(BigInt(1000000000), Planet.Jupiter);
  const saturn = years_during(BigInt(1000000000), Planet.Saturn);
  const uranus = years_during(BigInt(1000000000), Planet.Uranus);
  const neptune = years_during(BigInt(1000000000), Planet.Neptune);
  console.log(`${earth.toFixed(2)} earth years is:`);
  console.log(`${mercury.toFixed(2)} Mercury years`);
  console.log(`${venus.toFixed(2)} Venus years`);
  console.log(`${mars.toFixed(2)} Mars years`);
  console.log(`${jupiter.toFixed(2)} Jupiter years`);
  console.log(`${saturn.toFixed(2)} Saturn years`);
  console.log(`${uranus.toFixed(2)} Uranus years`);
  console.log(`${neptune.toFixed(2)} Neptune years`);
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #6: Sublist')
  console.log('*************************************************')
  const sub = sublist([2, 3], [1, 2, 2, 3]);
  const superlist = sublist([1, 2, 2, 3], [2, 3]);
  const equal = sublist([1, 2, 3], [1, 2, 3]);
  const uneqaul = sublist([1, 2], [2, 3]);
  console.log('Comparisons type for [2, 3] & [1, 2, 2, 3]: ', Comparison[sub]);
  console.log('Comparisons type for [1, 2, 2, 3] & [2, 3]: ', Comparison[superlist]);
  console.log('Comparisons type for [1, 2, 3] & [1, 2, 3]: ', Comparison[equal]);
  console.log('Comparisons type for [1, 2] & [2, 3]: ', Comparison[uneqaul]);
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #7: Minefield')
  console.log('*************************************************')
  const print_mf = (mf: string[]) => {
    for (const row of mf) {
      console.log(`"${row}"`);
    }
  }
  const minefield = [
    '  * ',
    '    ',
    ' *  ',
    '  **',
    ' *  ',
    '    ',
  ];
  print_mf(minefield);
  console.log('______');
  print_mf(annotate(minefield));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #8: Luhn Algorithm')
  console.log('*************************************************')
  const luhn_input = '234 567 891 234';
  console.log('luhn input: ', luhn_input);
  console.log('is valid: ', luhn(luhn_input));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #9: Armstrong Numbers')
  console.log('*************************************************')
  console.log('is 153 an armstrong number: ', is_armstrong_number(153));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #10: Beer Song')
  console.log('*************************************************')
  console.log(sing(4, 0));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #11: Difference of Squares')
  console.log('*************************************************')
  console.log('difference of squares 1..10', difference(10));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #12: Grains')
  console.log('*************************************************')
  console.log('grains on square 32', grains_square(32));
  console.log('grains on entire board', grains_total());
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #13: Leap')
  console.log('*************************************************')
  console.log('Is year 1970 a leap year: ', is_leap_year(BigInt(1970)));
  console.log('Is year 2000 a leap year: ', is_leap_year(BigInt(2000)));

  console.log('*************************************************')
  console.log('Excercise #14: Nth Prime')
  console.log('*************************************************')
  console.log('The 5th prime number is ', nth(4));
  console.log('The 22nd prime number is ', nth(21));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #15: Prime Factors')
  console.log('*************************************************')
  console.log('Prime factors of 625 should be [5, 5, 5, 5]', factors(BigInt(625)));
  console.log('Prime factors of 901255 should be [5, 17, 23, 461] ', factors(BigInt(901255)));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #16: Build Proverb')
  console.log('*************************************************')
  console.log('Build proverb of with: ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]');
  console.log(build_proverb(['nail', 'shoe', 'horse', 'rider', 'message', 'battle', 'kingdom']))
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #17: Raindrops')
  console.log('*************************************************')
  console.log('Raindrops 21:', raindrops(21));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #18: Raindrops')
  console.log('*************************************************')
  console.log('Sum of multiples:', sum_of_multiples(33, new Uint32Array([3, 5])))
  console.log('')


  console.log('*************************************************')
  console.log('Excercise #19: Bob')
  console.log('*************************************************')
  console.log("HEY BOB, HOW ARE YOU?");
  console.log('Bob: ', bob('HEY BOB, HOW ARE YOU?'));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #20: Highscores')
  console.log('*************************************************')
  console.log('Highscores not WASM\'d')

  console.log('*************************************************')
  console.log('Excercise #21: Matching Brackets')
  console.log('*************************************************')
  const bracket_input = '"\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \ \\end{array}\\right)";'
  console.log(`is balanced: ${bracket_input}`, brackets_are_balanced(bracket_input));

  console.log('*************************************************')
  console.log('Excercise #22: Collatz Conjecture')
  console.log('*************************************************')
  console.log('Collatz steps 16:', collatz(BigInt(16)));
  console.log('Collatz steps 1000000:', collatz(BigInt(1000000)));

  console.log('*************************************************')
  console.log('Excercise #23: Diffie-Hellman')
  console.log('*************************************************')
  const diffie_n = BigInt(5);
  const diffie_mod = BigInt(23);
  const diffie_private_key = private_key(BigInt(6131));
  const diffie_pub_key = public_key(diffie_mod, diffie_n, diffie_private_key);
  const diffie_secret = secret(diffie_mod, diffie_pub_key, diffie_private_key);
  console.log('diffie - public | private | secret', diffie_pub_key, diffie_private_key, diffie_secret);
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #24: Series')
  console.log('*************************************************')
  console.log('partition 4959303045 by 3:', series('4959303045', 3));
  console.log('')

  console.log('*************************************************')
  console.log('Excercise #25: Kindergarten Garden')
  console.log('*************************************************')
  const diagram = `VRCGVVRVCGGCCGVRGCVCGCGV
  VRCCCGCRRGVCGCRVVCVGCGCV`;
  console.log("Bob's plants: ", plants(diagram, "Bob"));
})
