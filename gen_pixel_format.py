import re

from jinja2 import Environment, FileSystemLoader


def main():
    names = []
    env = Environment(loader=FileSystemLoader("./templates"))
    template = env.get_template("pixel_format.rs.j2")
    with open("mvs-sys/src/lib.rs") as f:
        for line in f.readlines():
            m = re.match(
                r"pub const MvGvspPixelType_PixelType_Gvsp_([^ ]+): MvGvspPixelType = \d+;", line)
            if m is None:
                continue
            names.append(m.group(1))

    print(template.render(names=names))


if __name__ == "__main__":
    main()
