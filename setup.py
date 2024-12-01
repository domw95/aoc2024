from jinja2 import Environment, FileSystemLoader, select_autoescape
import sys
import subprocess

# Run with no arg to set repo back to default


try:
    year = sys.argv[1]
    lib_year = year
except IndexError:
    year = ""
    lib_year = "2022"


env = Environment(
    loader=FileSystemLoader(searchpath="templates"),
    autoescape=select_autoescape()
)

lib_template = env.get_template("lib.rs.j2")
main_template = env.get_template("main.rs.j2")
cargo_template = env.get_template("Cargo.toml.j2")

with open("src/lib.rs", "w") as file:
    file.write(lib_template.render(year=lib_year))

with open("src/main.rs", "w") as file:
    file.write(main_template.render(year=year))

with open("Cargo.toml", "w") as file:
    file.write(cargo_template.render(year=year))

day_template = env.get_template("day.rs.j2")

for day in range(1, 26):
    # try download input
    subprocess.run(["cargo", "aoc", "input", "-y", lib_year, "-d", str(day)])
