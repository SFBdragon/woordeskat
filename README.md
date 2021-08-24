# woordeskat
Console app to improve your Afrikaans vocab.

This app will test you on Afrikaans to English translations and vice versa on common Afrikaans words and vocabulary related to the essay topics of AS 2021 Afrikaans as a second language syllabus (and more). It will tetest you on vocabulary you get wrong in order to aid memorization. It also uses the levenshtein string distance algorithm so that if you're only a single letter off, it will accept your answer (this is helpful for special characters, for instance).

Please note that while I've spent a lot of time cleaning up the database, it still likely has numerous errors, and could always be improved or added to (see below if you'd like to contribute). Thanks to Matthew for helping me with some of the cleanup process. I will be adding to the database often. In order to update your woordeskat database (if you'd like), access the 'wsdb.txt' file above, and copy the contents (ctrl+a, ctrl+c) into the 'wsdb.txt' file on your computer (ctrl+a, ctrl+v).

For those unaquainted with Consoles/Github/Rust dev/etc: 
1. Head over to the right/down and locate the releases section.
2. Find the latest release (should be the only one you see).
3. Expand (if not already) the Assets section and download woordeskat_uncoloured.zip
4. Extract the files
5. Double click woordeskat.exe
6. If you experience any problems, please let me know.

A general guide for tinkerers:
- There is a 'colored' and an 'uncolored' version of the app. The former requires a modern terminal emulator to render to text in colour properly (`cmd.exe` and builtin `powershell.exe` are not modern enough, I recommend [Windows Terminal](https://www.microsoft.com/store/productId/9N0DX20HK701)). The latter version will work with anything.
- The `wsdb.txt` file is a database with all the translations. It probably still contains errors. If you'd like to submit changed tranlations, more synonyms/alternatives, or new tranlations; please submit a pull request. To do this, create a github account, edit the `wsdb.txt` in the repository, and create a pull request and/or issue.
- `wsdb.txt` is formatted to have one translation per line delimited by a '.', duplicate a translation and modify one side to add alternative tranlations. Avoid duplicating entire translations, running the app with duplicate translations will cause it to notify you of any duplicates it found.
- This is a basic console app built in Rust. Use `Cargo` to build it yourself if you'd like. Colored version is built by default, to build the uncolored version, specify the `--no-default-features` flag to Cargo. The output program files will be in the `target` directory, but `wsdb.txt` will not be copied over, and you will have to do this manually to create a standalone executable. Feel free to poke around the code or steal it (subject to the license included in the repository) if you wish.
