from pathlib import Path
import sys

from PIL import Image


def main() -> None:
    frames_dir = Path(sys.argv[1])
    output_path = Path(sys.argv[2])
    frames = [Image.open(path).convert("P", palette=Image.ADAPTIVE) for path in sorted(frames_dir.glob("*.png"))]
    if not frames:
        raise SystemExit("no PNG frames found")

    output_path.parent.mkdir(parents=True, exist_ok=True)
    frames[0].save(
        output_path,
        save_all=True,
        append_images=frames[1:],
        duration=1200,
        loop=0,
        optimize=False,
    )


if __name__ == "__main__":
    main()
