# Notatki

1: Element może być trzymany lokalnie i służyć jako zwykła notatka.
2: Element może być również upubliczniony, a wiec być możliwy do wyszukania.
3: Element może być w końcu dotowany, a więc posiadać swoją [Reaktywność].

## Reaktywność
Elementy mogę być ważne i mniej ważne, nie można zmuszać nikogo aby utrzymywał wszystkie elementy, często będzie to nawet prawie niemożliwe, a napewno powodowałaby centralizację i niepotrzebną duplikację elementów sieci.
Potrzebna jest metoda klasyfikacji elementów.
	* Jedną z nich może być liczba powtórzeń w sieci, a więc jej popularnośc, wartościowe elementy mogą nabierać wartości społecznej, będą wtedy współdzielone, będzie to jednak proces powolny i długotrwały.
	* Bezpośrednie dotowanie wiadomości daje możliwość szybkigo rozprzestrzenianie się elementu o wątpliwej wartości merytorycznej, szybciej następi również jej weryfikacja i jej ocena pożyteczności.

	Aby dokonywać dotacji, klient musi utworzyć swój kanał płatności, który będzie przypisany do jego konta (ze względu na konieczność dokonania kosztownej transakcji w głównym łańcuch podczas otwierania kanału płatności, kanał jest przypisywany do klienta, nie zaś do samego elementu)
	* Dokonanie dotacji odbywa się poprzez sieć ln, na drugiej warstwie bitcoina i sprawia, że jest tworzony kanał płatności. Pierwsza dotacja (umożliwienie klientowi dokonywania dotacji) może w związku z tym odbywac się dłużej i jest obarczona kosztami transakcyjnymi utworzenia kanału płatności.
	* Dotacje inna niż pierwsza powinny odbywajać się błyskawicznie.
	* Element wypromieniowuje swoją wartość w równych odstępach czasu, podczas dotacji oprócz przekazywanej wartości należy określić czas połowicznego rozpadu elementu.
		- Element dotowany na 100 000 satoshi i czasem połowiznego rozpadu 1 rok oznacza, że po roku zostanie wypromieniowana warość 50 000 Satoshi, idąc dalej wartość pozostała z dotacji 2 latach, po wypromieniowaniu 75 000 satoshi, wyniesie 25 000 satoshi (100 000 - 50 000 - 25 000).
		- Utrata wartości przez element w czasie nazywamy mocą elementu.
	* Beneficjentami wypromieniowującej wartości są działające instancję "strażaków", którzy współdzielą dany element, każdy oddzielną jego kopią.
		- Strażacy muszą posiadać kanał płatności związany z ich kontem.
		- Wyptomieniowana wartość danego elementu podzielona przez jego rozmiar nazywamy wypromieniowywaną wartością właściwą.
		- Wypromieniowana porcja wartości właściwa dzielona po równo na ilość aktywnych strażaków współdzielących dany element nazywamy reakywnością elementu.
		- Może się zdażyć, że mała reaktywność elementu w dużej swojej części jest pochłaniana przez koszty transferowe, a największym jej beneficjentem jest najbliższy węzeł konta który dokonał dotacji.
		- * [w zależności od implementacji ln] Wypromieniowywana wartość następuje w równych odstępach czasu tylko w przypadku dużej reaktywności (zdolnej do podziału po pokryciu kosztów transferowych do wszystkich strażaków)

	Konsekwencją tak wypracowanego schematu jest zachęta (pochłanianie wartości) do bycia możliwie blisko i współdzelenie elementu ze żródła gdzie element jest wysoko dotowany i posiada małym czas połowicznego rozpadu, a ponadto jest maksymalnie mały i jest słabo rozpowszechniony, gdzyż wtedy wyproniewowywana wartość przypadająca na indywidualnego strażaka jest największa, a więc posiada możliwie dużą reaktywność.
	W interesie strażaka jest współdzielenie maksymalnej liczby elementów o największej reaktywności, dzięki wprowadzeniu reaktywności jesto to bardzo proste i możliwe do zautomatyzowania, a jedynym ograniczeniem jest posiadana przez strażaka przestrzeń na składowanie elementów.

### Reaktywność realna
	W realnym układzie element może być dotowany przez więcej niż jeden węzeł, dotacja druga i następne zwiększają reaktywność elementu, ale również sprawiają, że wypromieniowywana wartość ulega denectralizacji i dokonuje się z różnych miejsc w sieci. Przy odpowieniednio dużej liczbie dotacji następującej z niezależnych źródeł, miejsce w sieci posiadanego przez strażaka węzła traci znaczenie, a element sam w sobie staje się bardziej niezależny od swojego autora (pierwszej dotacji), a on sam traci swoją uprzywilejowaną pozycję do pochłaniania proporcjonalnych większej wartości od innych uczestników sieci. Tak jak dzisiaj,ani sam autor, ani jego następcy takich wynalazków jak ogień, koło, czy silnik spalinowy nie są chronieni żadnym prawem intelektualnym/autorskim, gdyż nie ma to sensu.
	Samo prawo do własności intelektualnej wydaje się być pozbawione sensu dla rozpowszechnionego już elementu, użytkownik tylko sam i zawsze może zabezpieczyć swoją własność intelektualną, po prostu jej nie publikując, a w skrajnych przypadkach nawet nie artykułując. Prawo do własności (czerpania specjalnych korzyści) intelektualnej i wiedza, która już została upublicznona, bezpowrotnie zanika, co nie znaczy, że wiedza ta może zawierać w sobie symboliczną informację o autorstwie.


## Relacja
	Relacja zachodzi między dwoma różnymi elementami.
	Relacje pierwotne:
		- element należy do zbioru
		- element nie należy do zbioru
		- element jest równoważny innemu elementowi
		- element nie jest równoważny innemu elementowi

	Przykładem bardziej złożonej relacji jest "autorstwo"
	Twórcą elementu jest Bob, jednak Bob mógł, nawet nieświadomie popełnić plagiat, a więc kwestia jego autorswa może zostać podważona.

	Interesującą relacją jest podważenie prawdziwości elementu, jako, że relacja zachodzi między dwoma elementami, ta relacja oprócz elementu podważanego musi odnościć się do elementu zawierającego dowód lub bardziej lub mniej merytoryczną argumentację. Argumentacja może ta być przekonująca lub nie i to od indywidualnego użytkownika zależy czy ją uzna, czy odrzuci.

	Wykazanie sprzeczności to kolejna interesująca relacja, która może być zarówno prosta jak i złożna z całej grupy elementów, wykazjującej tą sprzeczność poprzez inne relacje.

## Zbiór
	Zbiór jest również elementem, a w jego skład wchodzą jego podelementy z relacjami zachodzącymi między nimi.


