To release a new version of pysyntect:
1. git fetch upstream && git checkout upstream/master
2. Close milestone on GitHub
3. git clean -xfdi
4. Update CHANGELOG.md with loghub
5. git add -A && git commit -m "Update Changelog"
6. Update release version in ``Cargo.toml`` (set release version, remove 'dev0')
7. git add -A && git commit -m "Release vX.X.X"
8. maturin build
9. maturin publish
10. git tag -a vX.X.X -m "Release vX.X.X"
11. Update development version in ``Cargo.toml`` (add '-dev0' and increment minor, see [1](#explanation))
12. git add -A && git commit -m "Back to work"
13. git push upstream master
14. git push upstream --tags

[<a name="explanation">1</a>] We need to append '-dev0', as Cargo does not support the '.dev0'
syntax.
