#!/usr/bin/env python3
"""
CI-EN 記事一覧を取得して assets/articles.json に出力するスクリプト。
GitHub Actions のビルドステップで実行する。
"""

import json
import re
import sys
import urllib.request
from html.parser import HTMLParser
from datetime import datetime

CREATOR_ID = 7509
URL = f"https://ci-en.dlsite.com/creator/{CREATOR_ID}/article"
OUTPUT = "assets/articles.json"


class CienParser(HTMLParser):
    """CI-EN の記事一覧ページから記事情報を抽出する簡易パーサー。"""

    def __init__(self):
        super().__init__()
        self.articles = []
        self._current = {}
        self._in_title = False
        self._depth_card = 0
        self._in_card = False
        self._in_date = False

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)

        # カード要素の検出
        cls = attrs.get("class", "")
        if "c-postedArticle" in cls:
            self._in_card = True
            self._current = {}

        if not self._in_card:
            return

        # サムネイル画像
        if tag == "img" and "v-image" in cls:
            src = attrs.get("src") or attrs.get("data-src", "")
            alt = attrs.get("alt", "")
            if src and "article" in src or "cover" in src:
                self._current["thumbnail"] = src
                if not self._current.get("title"):
                    self._current["title"] = alt

        # 記事リンク
        if tag == "a" and "c-cardLink" in cls:
            href = attrs.get("href", "")
            if f"/creator/{CREATOR_ID}/article/" in href:
                self._current["url"] = href

        # 日付
        if tag == "p" and "e-date" in cls:
            self._in_date = True

        # タイトル
        if tag == "h2" and "e-title" in cls:
            self._in_title = True

    def handle_endtag(self, tag):
        if tag == "div" and self._in_card:
            # 必須フィールドが揃っていれば保存
            if self._current.get("url") and self._current.get("title"):
                self.articles.append(dict(self._current))
            self._in_card = False
            self._current = {}

    def handle_data(self, data):
        data = data.strip()
        if not data:
            return
        if self._in_date:
            self._current["date"] = data[:10].replace("/", "/")
            self._in_date = False
        if self._in_title:
            self._current["title"] = data
            self._in_title = False


def fetch_articles():
    req = urllib.request.Request(
        URL,
        headers={
            "User-Agent": (
                "Mozilla/5.0 (compatible; StaticSiteBuilder/1.0; "
                "+https://github.com/nemuinari/website)"
            )
        },
    )
    with urllib.request.urlopen(req, timeout=15) as resp:
        html = resp.read().decode("utf-8", errors="replace")

    parser = CienParser()
    parser.feed(html)

    # 重複除去（URLをキーに）
    seen = set()
    unique = []
    for a in parser.articles:
        if a["url"] not in seen:
            seen.add(a["url"])
            unique.append(a)

    return unique


def main():
    print(f"Fetching CI-EN articles from {URL} ...")
    try:
        articles = fetch_articles()
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        # フォールバック: 空リストで続行（ビルドを止めない）
        articles = []

    print(f"  -> {len(articles)} articles found")

    payload = {
        "generated_at": datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ"),
        "articles": articles,
    }

    with open(OUTPUT, "w", encoding="utf-8") as f:
        json.dump(payload, f, ensure_ascii=False, indent=2)

    print(f"  -> written to {OUTPUT}")


if __name__ == "__main__":
    main()
