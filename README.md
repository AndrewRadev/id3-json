# ID3-JSON

```
% cargo run attempt_1.mp3
{"album":"Echoes From The Past","artist":"Christiaan Bakker","comment":"http://www.jamendo.com Attribution 3.0 ","genre":"(255)","title":"Elevator Music Attempt #1","track":null}

% echo '{ "title": "Updated Title" }' | cargo run attempt_1.mp3 --write

% cargo run attempt_1.mp3
{"album":"Echoes From The Past","artist":"Christiaan Bakker","comment":"http://www.jamendo.com Attribution 3.0 ","genre":"(255)","title":"Updated Title","track":null}
```
