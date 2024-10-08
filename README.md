# Bakaláři Tracker
**CZ**  
Taky nenávidíte, že bakaláři furt nefungujou? Kdo ne. Proto jsem vytvořil tuto aplikaci, která Loguje status vašich Školních bakalářů, aby jste poté mohli jít upálit školního IT specialistu  
**EN**  
This app tracks, whether the Czech school system "Bakaláři" is working or not

## Instalace a používání
- Stáhni a extrahuj .zip Soubor z vydání.
  - Jsou dostupné i samotné soubory, ale doporučuji .zip metodu
- Do `.env` souboru se zadá url adresa bakalářů školy, a jak často se má stav zaznamenávat.
- Je potřeba se ujistit že v Adresáři programu existuje Adresář `tmp/`
- Spustí se Binary.
- Program vytvoří Daemon, který pracuje v pozadí.
- Program vytvoří CSV soubor. S tím si dělejte, co chcete.
  - Program při následujících spuštění použije stejný soubor, případně vytvoří nový, pokud bude odstraněn

**TODO**:
- Vytvořit Adresář `tmp/` pokud neexistuje
- Něco, co mi ty data zkrášlí trochu, courtesy of [@XdaZCZ](https://github.com/XdaZCZ)
