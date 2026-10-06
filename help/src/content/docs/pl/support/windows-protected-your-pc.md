---
title: "System Windows ochronił ten komputer: uruchom instalator Moonpool mimo to (SmartScreen)"
description: "Windows SmartScreen pokazuje Windows protected your PC przy uruchamianiu moonpool.exe. Dlaczego tak się dzieje, jak wybrać More info, a potem Run anyway, i co sprawdzić najpierw."
---

Po uruchomieniu pobranego `moonpool.exe` system Windows może pokazać niebieskie okno zatytułowane
**Windows protected your PC** („System Windows ochronił ten komputer”), z tekstem „Microsoft Defender
SmartScreen prevented an unrecognized app from starting. Running this app might put your PC at
risk.” (SmartScreen uniemożliwił uruchomienie nierozpoznanej aplikacji; jej uruchomienie może
narazić komputer na ryzyko).

## Dlaczego się pojawia

SmartScreen ostrzega przed programami, które są nowe lub których nie widział uruchamianych na wielu
komputerach. `moonpool.exe` nie jest podpisany cyfrowo, więc Windows nie ma wydawcy, któremu
mógłby zaufać, i może pokazać ostrzeżenie przy pierwszym uruchomieniu. To kontrola reputacji, a nie
stwierdzenie, że plik jest złośliwy.

## Co zrobić

1. W oknie należy kliknąć **More info** („Więcej informacji”). Wydawca jest pokazany jako „Unknown
   publisher” („Nieznany wydawca”).
2. Należy kliknąć **Run anyway** („Uruchom mimo to”). Otworzy się karta instalacji. Zob.
   [Instalacja](/pl/getting-started/install/).

Jeśli chcesz najpierw zachować ostrożność, należy pobierać wyłącznie z oficjalnej witryny
Moonpool lub jego wydań w serwisie GitHub i sprawdzić, czy nazwa pliku to `moonpool.exe`.

## Jeśli nie ma przycisku Run anyway

Na niektórych komputerach zarządzanych administrator wyłącza tę opcję i nie ma przycisku **Run
anyway**. Zapytaj administratora albo użyj komputera, którym zarządzasz. Plik, który
trafił w pobranym archiwum zip, może też mieć blokadę: należy kliknąć plik prawym przyciskiem,
wybrać **Właściwości**, zaznaczyć **Odblokuj**, jeśli się pojawi, potem **OK** i uruchomić go
ponownie.

## Ostrzeżenia programu antywirusowego

Nowy niepodpisany plik exe, który kopiuje się do profilu użytkownika i zastępuje się przy
aktualizacji, może też wzbudzić alarm programu antywirusowego. Jeśli Twój program blokuje lub
poddaje kwarantannie `moonpool.exe`, należy zezwolić na niego w folderze `.moonpool`. Zob.
[Windows](/pl/platforms/windows/#przed-uruchomieniem).

## Zobacz także

- [Instalacja](/pl/getting-started/install/)
- [Windows](/pl/platforms/windows/)
- [Instalator pokazuje błąd](/pl/support/troubleshooting/#instalator-pokazuje-błąd)
