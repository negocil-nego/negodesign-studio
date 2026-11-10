Para este projecto irás ótmizar todos códigos seguindos essas regras:

-Máximo são 100 linhas
-Os componentes irão seguir o padrão de flutter utilizar a programação declarativa
como exemplo o "CardMediaBaseProps" que todos propriedades são objectos composto como
recomenda o "Object Calisthenics" nas suas 9 regras de Jeff Bay no livro The ThoughtWorks Anthology,
-Para os componentes que são de linguagem ts, como vue, react, svelte, deves criar o padrão devem usar
o mesmo Propriedade, todo que é lógica de para os componentes deves criar a lógica fora em hook, de forma
que possas reaproveitar para as os três tecnologias (vue, react, svelte), arranja forma de reaproveitar o Máximo
de código porque essas tecnologias são frontend em TS. Um mal exemplo de que não quero é o que esta no componente
src\project\ui\organisms\data-table\shared\data-table-filter\ui
-Para o flutter será diferente porque não é uma tecnologia equivalente porque é uma linguagem diferente e um
tipo de desenvolvimento mobile a idea é fazer a equivalencia para flutter em uma aplicação mobile, de aqui deves
ser extremanente com a quantidade de linhas que não deve passar de 100 linhas. Olha podes criar componente novos ou
diferente de ts para ser exclucivo de flutter, não gosto de componentes internos seram criados em mesma classe caso precisares
criar podes criar uma pasta flutter dentro de ui que irás colocar a componentes privados e irás actualizar o arquivo,
dependence.json.
-Se baseando na ideia de ThemaData de Flutter e StyleComponente de React Nativa, encontra uma forma melhor de estilazação dos
dos componenes além do que seguido neste projecto que utiliza o "src\project\data\style.component.ts",
eu quero que da forma que estilizo deve ser jeito sempre por um arquivo de configuração externo não tenho que entrar no componente
para estilizar de modo se querer alterar alguma coisa não tenho que procurar os componente sempre foi no arquivo de estelização
a idea é ter uma arquivo intermediário que eu possa converte em ThemaData para Flutter e StyleComponente para as tecnologia de
(vue, react, svelte).
-Do mesmo jeito que terás um arquivo de estelização terás um arquivo icones que para a tecnologias (vue, react, svelte) utilizas
as classes do hugeicons e para Flutter ele carregar os arquvios svg.

Para cada componente faça a documentação dele, de forma que tenha um README.md a acada componente a explicar e que tenha tenha um README.md
para treinamento de IA que explicar do jeito que deve ser construído os componentes desta aplicação.

Para funcionalidade de copiar deves para projectos de "tecnologias (vue, react, svelte)" que trabalham com frameworks
SvelteKit, Nuxt, Next, Vue, React a ideia é sempre copiar tudo na colocar na pasta "/src" criar a pasta negodesign com os componentes
(semelhante com shacdcn faz) e seus próprios arquivos de configuração como de estilização e icones todo de de importe é por pacote tanto
em flutter também apenas tenho que chamar o "negodesign", tantos hooks, componente tudo virá dele.
para o flutter irás colocar dentro do "lib".
