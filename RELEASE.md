To release a new version of pysyntect:
1. git fetch upstream && git checkout upstream/master
2. Close milestone on GitHub
3. git clean -xfdi
4. Update CHANGELOG.md with loghub
5. git add -A && git commit -m "Update Changelog"
6. Update release version in ``Cargo.toml`` (set release version, remove 'dev0')
7. git add -A && git commit -m "Release vX.X.X"
8. git tag -a vX.X.X -m "Pysyntect release vX.X.X"
9. maturin build
10. maturin publish
11. Update development version in ``Cargo.toml`` (add '-dev0' and increment minor)
12. git add -A && git commit -m "Update development version to vX.X.X-dev0"
13. git push upstream --tags
14. git push HEAD upstream:master
