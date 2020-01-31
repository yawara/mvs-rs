import re

from jinja2 import Environment, FileSystemLoader


def snake_to_camel(snake_case):
    words = snake_case.lower().split('_')
    return ''.join([word.title() for word in words])


def main():
    names = []
    raws = []
    env = Environment(loader=FileSystemLoader("./templates"))
    template = env.get_template("error.rs.j2")
    with open("mvs-sys/src/lib.rs") as f:
        for line in f.readlines():
            m = re.match(r"pub const MV_E_([^ ]+): u32 = (\d+);", line)
            if m is None:
                continue
            raws.append(m.group(1))
            names.append(snake_to_camel(m.group(1)))

    print(template.render(names_and_raws=list(zip(names, raws))))


if __name__ == "__main__":
    main()
