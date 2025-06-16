from selenium import webdriver
import os
import csv
import argparse
from path import get_path


""" Fetches all provided URLs and saves their HTML content to files."""


def main(path: str):
    with open(path + "dataset.csv", "r") as file:
        reader = csv.DictReader(file)
        entries = list(reader)

        driver = webdriver.Chrome()

        for entry in entries:
            url = entry["url"]
            filepath = path + "/" + get_path(url)
            os.makedirs(os.path.dirname(filepath), exist_ok=True)

            if not os.path.exists(filepath):
                driver.get(url)

                with open(filepath, "w") as f:
                    f.write(driver.page_source)

        driver.quit()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Fetch URLs and save HTML content")
    parser.add_argument(
        "path", help="Path to dataset directory (e.g., dataset/wikipedia/)"
    )

    args = parser.parse_args()
    main(args.path)
