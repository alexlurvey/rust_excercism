import { getDayOfYear } from 'date-fns/getDayOfYear';
import init, {
  after,
  anagram_for,
  reverse,
  Clock,
} from "../pkg/wasm_playground.js";

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
})
