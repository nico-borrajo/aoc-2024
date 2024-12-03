
# ------------- get day -------------
if [ -d "inputs" ] ; then
    prev_day=$(ls inputs | tail -n 1)
    dayn=$(($prev_day + 1))
else
    mkdir inputs
    dayn="1"
fi

# $day contains formated day (leading 0)
day=$(printf "%02d" "$dayn")


# ------------- obtain the inputs -------------
mkdir inputs/$day
curl -b "session=$AOC_TOKEN"  https://adventofcode.com/2024/day/$dayn/input -o inputs/$day/input.txt -s
touch inputs/$day/test_input.txt # this file has to be filed manualy


# ------------- add lib code -------------

# create module in lib.rs
echo "pub mod day_$day;" >> src/lib.rs

# create module directory
mkdir -p src/day_$day

# create mod.rs file
echo "
pub mod part1;
pub mod part2;" > src/day_$day/mod.rs


# create part1.rs file
echo "
pub fn part1(input: &str) -> isize {
    // your code goes here...
    let _ = input;
    0
}" > src/day_$day/part1.rs

# create part2.rs file
echo "
pub fn part2(input: &str) -> isize {
    // your code goes here...
    let _ = input;
    0
}" > src/day_$day/part2.rs


# ------------- add bin code -------------
echo "fn main() {
    aoc_2024::run(\"$day\", aoc_2024::day_$day::part1::part1);
}" > src/bin/$day-1.rs

echo "fn main() {
    aoc_2024::run(\"$day\", aoc_2024::day_$day::part2::part2);
}" > src/bin/$day-2.rs
