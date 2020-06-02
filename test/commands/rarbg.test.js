const {expect, test} = require('@oclif/test')

describe('rarbg', () => {
  test
  .stdout()
  .command(['rarbg'])
  .it('runs hello', ctx => {
    expect(ctx.stdout).to.contain('hello world')
  })

  test
  .stdout()
  .command(['rarbg', '--name', 'jeff'])
  .it('runs hello --name jeff', ctx => {
    expect(ctx.stdout).to.contain('hello jeff')
  })
})
