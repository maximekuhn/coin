# Translations management

## Add a new locale
step 1: create a new file for the locale:
```shell
cp ./src/locale/messages.xlf ./src/locale/messages.<LOCALE_CODE>.xlf
```

step 2: To translate all texts, execute the `prompt` script:
```shell
python3 ./scripts/i18n_prompt.py --target <LOCALE_CODE>
```

step 3: Copy the output of the `prompt` script into [ChatGPT](https://chatgpt.com/) (or any other LLM)

step 4: Import the LLM output using the `import` script:
```shell
clipboard-paste | python3 ./scripts/i18n_import.py --target <LOCALE_CODE>
```
> use --dry-run and --verbose flags to preview changes before applying them

step 5: carefully review translations

## Synchronize translations
step 1: extract translations using [XLIFF v2.0](https://docs.oasis-open.org/xliff/xliff-core/v2.0/xliff-core-v2.0.html) format:
```shell
ng extract-i18n --format xlf2 --output-path src/locale
```

step 2: translate all texts by following steps 2 to 5 in section [Add new locale](#adding-new-locale)

step 3: run the `sync` script to delete old translations and update modified ones:
```shell
python3 ./scripts/i18n_sync.py --target <LOCALE_CODE>
```

step 4: carefully review updated translations
