# Mindfull Pomodoro

```Brought you by Ian Simao```

&emsp; Já teve o grave problema de querer utilizar um app de pomodoro no seu archlinux (btw) com hyprland mas não encontrar um nativo?
Pois é, seu problema acaba agora! Com o Mindfull pomodoro você poderá organizar mais ainda a suas atividades diárias! Não perca tempo, nem fique de cabeça cheia!

``` md
@@@   @@@@@@   @@@  @@@        @@@@@@   @@@  @@@@@@@@@@    @@@@@@    @@@@@@   
@@@  @@@@@@@@  @@@@ @@@       @@@@@@@   @@@  @@@@@@@@@@@  @@@@@@@@  @@@@@@@@  
@@!  @@!  @@@  @@!@!@@@       !@@       @@!  @@! @@! @@!  @@!  @@@  @@!  @@@  
!@!  !@!  @!@  !@!!@!@!       !@!       !@!  !@! !@! !@!  !@!  @!@  !@!  @!@  
!!@  @!@!@!@!  @!@ !!@!       !!@@!!    !!@  @!! !!@ @!@  @!@!@!@!  @!@  !@!  
!!!  !!!@!!!!  !@!  !!!        !!@!!!   !!!  !@!   ! !@!  !!!@!!!!  !@!  !!!  
!!:  !!:  !!!  !!:  !!!            !:!  !!:  !!:     !!:  !!:  !!!  !!:  !!!  
:!:  :!:  !:!  :!:  !:!  :!:      !:!   :!:  :!:     :!:  :!:  !:!  :!:  !:!  
::  ::   :::   ::   ::  :::  :::: ::    ::  :::     ::   ::   :::  ::::: ::  
:     :   : :  ::    :   :::  :: : :    :     :      :     :   : :   : :  :  
        .n                   .                 .                  n.
.   .dP                  dP                   9b                 9b.    .
4    qXb         .       dX                     Xb       .        dXp     t
dX.    9Xb      .dXb    __                         __    dXb.     dXP     .Xb
9XXb._       _.dXXXXb dXXXXbo.                 .odXXXXb dXXXXb._       _.dXXP
9XXXXXXXXXXXXXXXXXXXVXXXXXXXXOo.           .oOXXXXXXXXVXXXXXXXXXXXXXXXXXXXP
'9XXXXXXXXXXXXXXXXXXXXX'~   ~'OOO8b   d8OOO'~   ~'XXXXXXXXXXXXXXXXXXXXXP'
    '9XXXXXXXXXXXP' '9XX'          '98v8P'          'XXP' '9XXXXXXXXXXXP'
        ~~~~~~~       9X.          .db|db.          .XP       ~~~~~~~
                        )b.  .dbo.dP''v''9b.odb.  .dX(
                    ,dXXXXXXXXXXXb     dXXXXXXXXXXXb.
                    dXXXXXXXXXXXP'   .   '9XXXXXXXXXXXb
                    dXXXXXXXXXXXXb   d|b   dXXXXXXXXXXXXb
                    9XXb'   'XXXXXb.dX|Xb.dXXXXX'   'dXXP
                    ''      9XXXXXX(   )XXXXXXP      ''
                            XXXX X.'v'.X XXXX
                            XP^X''b   d''X^XX
                            X. 9  ''   '  P )X
                            'b  '       '  d'
                            '             '
```

---

## Pre-requisitos

&emsp; Se voce ta aqui, provavelmente ja tem o basico rodando. Mas vai saber, segue a lista do que precisa ter instalado antes de sair compilando:

### Sistema

- **Arch Linux** (btw) ou qualquer distro que se preze
- **Hyprland** (ou outro compositor Wayland com suporte a Layer Shell)
- **PulseAudio** ou **PipeWire** (com `pactl` disponivel no PATH)

### Dependencias de build

```bash
# Arch Linux (btw)
sudo pacman -S gtk4 gtk4-layer-shell base-devel rust alsa-lib pkg-config

# Fedora
sudo dnf install gtk4-devel gtk4-layer-shell-devel alsa-lib-devel rust cargo pkg-config

# Ubuntu/Debian (se voce insiste)
sudo apt install libgtk-4-dev libgtk4-layer-shell-dev libasound2-dev pkg-config rustc cargo
```

---

## Init

&emsp; Clonou? Beleza. Agora e so buildar e sair usando:

```bash
# Clona o repo
git https://github.com/ianpsa/Pomodoro-for-hyprland.git
cd pomodoro-mindfullness

# Build em modo release (recomendado, senao fica lerdo)
cargo build --release
#ou
cargo install --path .

# Roda
./target/release/pomodoro-mindfullness
```

&emsp; O app vai criar automaticamente o arquivo de config em `~/.config/pomodoro-service/config.toml` na primeira execucao.

### Releases

&emsp; Se nao quer compilar na mao, da pra pegar o binario pronto na aba de [Releases](../../releases) do GitHub. Tem build pra:

- `linux x86_64` — o classico
- `linux aarch64` — pra quem ta no ARM

&emsp; Baixa, e `chmod +x`.

## Como utilizar

1. Clique no círculo ```para``` movê-lo de um canto a outro da tela.
2. Clique no 'X' com o botão esquerdo ```para``` deixá-lo no modo mínimo e com o botão direito ```para``` fechá-lo!
3. Clique no Brown Noise ```para``` escutar ruído marrom enquanto trabalha.