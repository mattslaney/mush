# mush
The the beginnings of a file and folder syncing application written in rust.

## Main Goals
 1. Compare source and destination directories to identify differences
    * Ability to check identical trees
    * Ability to check all source files exist on dest irrespective of tree
    * Ability to verify file integrity across directories
    * Can product manifest to do later

 2. Ability to synchronise source and destination directories
    * Bidirectional sync (conflicts prompted)
    * One-way move/copy
    * Can perform dryrun to verify actions
    * Can produce audit (that can be used for undo)

 3. Ability to replace duplicate files with a symbolic links or delete
    * Favouring shortest path
    * Favouring age of file (newest/oldest)

## Use Cases

##### Global Flags
 * `--ignore` Ignore file which works relative to the root of each directory specified
 * `--include` Opposite of ignore file, only include these files
 * `--quiet` Only show warning and errors (`--logging 2`)
 * `--verbose` Provide a verbose log output (`--logging 4`)
 * `--logging <level>` 0 = FATAL, 1 = ERROR, 2 = WARNING, 3 = INFO, 4 = DEBUG, 5 = TRACE
 * `--progress` Show progress
 * `--report <FILENAME>` Generate a report

### Scanning
`mush index <LIST_OF_PATHS>`
 * Scans the paths and creates an index mush can use to speed up operations

`mush index <LIST_OF_PATHS> --csv <FILE>`
 * Scans the paths and creates a csv file
 * basepath,filename,extension,type,owner,permissions,date_created,date_modified,size_bytes,sha256,seahash
 * type = file, symlink

### Checking
These are the commands that have no real effect

##### Global Checking Flags
`--manifest <FILENAME>`
 * Generate a manifest of actions

#### Diff
Fast check to see if directories match

`mush diff <LIST_OF_PATHS>`
 * Diff both directories and list missing or mismatching files across each
 * Strict check, expect directories to be identical
 * Return 0 for identical, return 1 for mismatching

`mush diff --src <LIST_OF_PATHS> --dst <PATH>`
 * List files on src missing from destination
 * Loose check, just expect file to exist somewhere (same name, same size)
 * Return 0 for no missing files, return 1 for missing files

#### Verify
Thorough check to see if directories match

`mush verify <LIST_OF_PATHS>` (alias of `mush diff <LIST_OF_PATHS> --verify`)
 * Verify the data matches across the directory paths provided
 * Strict check, expect directories to be identical
 * Return 0 for identical, return 1 for mismatching

`mush verify --src <LIST_OF_PATHS> --dst <PATH>` (alias of `mush diff --src <LIST_OF_PATHS> --dst <PATH> --verify`)
 * List files on src not matching any on destination
 * Loose check, just expect files to exist somewhere and contents to match
 * Return 0 for no matching data, return 1 for mismatching data

#### Clutter
Identifies clutter (duplicate files, empty directories)

##### Clutter Flags
 * `--recent` - favours ther most recently modified file (default)
 * `--shortpath` - favours the shortest path
 * `--newest` - favours the newest file
 * `--oldest` - favours the oldest file

`mush clutter <PATH>`
 * Lists out any clutter
 * Returns 0 if no clutter, returns 1 if cluttered

`mush clutter <PATH> --manifest`
 * Lists out any clutter
 * Creates manifest of files to act upon
 * Returns 0 if no clutter, returns 1 if cluttered

### Actions
These are the commands that have some effect or consequence

##### Global Action Flags

 * `--dryrun` Perform a dryrun and highlight for any potential issues
 * `--audit` Create an audit file listing every action that was performed

#### Sync
Command to keep directories in bidirectional sync (favouring most recently modified)

`mush sync <LIST_OF_PATHS>`
 * Synchronise all paths so they match

#### Copy
Command to copy files from source to destination (no files are copied from dest)

`mush copy <PATH> <PATH>`
 * Copy all files from source to destination

`mush copy --src <LIST_OF_PATHS> --dst <PATH>`
 * Copy all files from the source(s) to the destination
 * Place in a unique directory for each source

`mush copy --src <LIST_OF_PATHS> --dst <PATH> --merge`
 * Copy all files from the source(s) to the destination
 * Attempt to merge
 * Prompt to rename for duplicate filenames (where files differ)

`mush copy --src <LIST_OF_PATHS> --dst <PATH> --files`
 * Copy all files from the source(s) to the destination without directory structure
 * Prompt to rename for duplicate filenames

`mush copy --src <LIST_OF_PATHS> --dst <PATH> --files -y`
 * Copy all files from the source(s) to the destination without directory structure
 * yes, rename automatically

#### Move
Command to move files from source to destination (no files are moved from dest)

`mush move <PATH> <PATH>`
 * Move all files from source to destination

`mush move --src <LIST_OF_PATHS> --dst <PATH>`
 * Copy all files from the source(s) to the destination
 * Place in a unique directory for each source

`mush move --src <LIST_OF_PATHS> --dst <PATH> --merge`
 * Copy all files from the source(s) to the destination
 * Attempt to merge
 * Prompt to rename for duplicate filenames (where files differ)

`mush move --src <LIST_OF_PATHS> --dst <PATH> --files`
 * Copy all files from the source(s) to the destination without directory structure
 * Prompt to rename for duplicate filenames

`mush move --src <LIST_OF_PATHS> --dst <PATH> --files -y`
 * Copy all files from the source(s) to the destination without directory structure
 * yes, rename automatically

#### Do
Perform all the actions listed in the manifest
`mush do <MANIFEST>`

### Undo
Perform the reverse of all the actions listed in an audit
`mush undo <AUDIT>`
