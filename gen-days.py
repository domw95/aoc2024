from jinja2 import Environment, FileSystemLoader, select_autoescape
import sys
import subprocess

env = Environment(
    loader=FileSystemLoader(searchpath="templates"),
    autoescape=select_autoescape()
)


day_template = env.get_template("day.rs.j2")

for day in range(1, 26):
    code = day_template.render(day="{}".format(day))
    with open("src/day{}.rs".format(day), "w") as file:
        file.write(code)
