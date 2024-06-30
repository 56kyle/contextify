"""Command-line interface."""

import click


@click.command()
@click.version_option()
def main() -> None:
    """Contextify."""


if __name__ == "__main__":
    main(prog_name="contextify")  # pragma: no cover
