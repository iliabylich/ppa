# Personal PPA

```sh
curl -s --compressed "https://iliabylich.github.io/ppa/iliabylich_ppa.gpg" | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/iliabylich_ppa.gpg > /dev/null

sudo curl -s --compressed -o /etc/apt/sources.list.d/iliabylich_list_file.list "https://iliabylich.github.io/ppa/iliabylich_list_file.list"

apt update
```