# Bakaláři Tracker
**CZ**  
Taky nenávidíte, že bakaláři furt nefungujou? Kdo ne. Proto jsem vytvořil tuto aplikaci, která Loguje status vašich Školních bakalářů, aby jste poté mohli jít upálit školního IT specialistu  
**EN**  
This app tracks, whether the Czech school system "Bakaláři" is working or not

# Instalace a používání
- Stáhni a extrahuj .zip Soubor z vydání.
  - *Jsou dostupné i samotné binary soubory, ale doporučuji .zip metodu*
- Do `.env` souboru se zadá url adresa bakalářů školy, a jak často se má stav zaznamenávat.
- **Je potřeba se ujistit, že v `.env` souboru jsou zadané veškeré potřebné informace správně**
- Spustí se Binary.
- Program vytvoří Daemon, který pracuje v pozadí.
- Program vytvoří CSV soubor, do kterého průběžně ukládá data.  
  Nedoporučuji soubor přemisťovat když program běží, může dojít ke ztrátě dat.  
  S CSV souborem lze jinak libovolně manipulovat. V případě jeho odstranění program vytvoří nový, jinak píše stále do stejného

## Ukázkový `.env` soubor:
```
BAKALARI_URL = https://www.google.com
MS_SLEEP_BETWEEN_CHECKS = 600000
```
## Tree view správně připraveného adresáře:
```
bakalari-tracker/
├── bakalari-tracker.exe
├── .env
├── tmp/
│   └── //vytvoří se samo
└── log.csv //vytvoří se samo
```
## Building a debugging:
```
git clone https://github.com/jenyyk/bakalari-tracker.git
cd bakalari-tracker
cargo run
```

**TODO**:
- Něco, co mi ty data zkrášlí trochu, courtesy of [@XdaZCZ](https://github.com/XdaZCZ)
