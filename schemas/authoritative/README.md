# Authoritative source and coverage register

No actual standards, message layouts, or DFI/DUI dictionaries are present here.
`source-register.json` contains public catalog metadata from [S1] only.
`coverage.json` reports zero implemented/verified standard messages and unknown
inventory denominators.

`workspace_source_status=not_acquired` describes this generated workspace, not
what is available inside the user's approved environment. Local availability
there is explicitly unknown. Null hashes and local locators are intentional.

Populate actual source/evidence details only in the approved environment. Do
not put controlled documents into this folder merely because it exists, and
do not interpret `.gitignore` as an access-control mechanism. Review both source
and derived-schema handling before choosing a repository or artifact store.

[S1] https://quicksearch.dla.mil/qsDocDetails.aspx?ident_number=123964
