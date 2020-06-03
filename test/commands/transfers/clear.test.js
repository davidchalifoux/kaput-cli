const {expect, test} = require('@oclif/test')

describe('transfers:clear', () => {
  test
  .stdout()
  .command(['transfers:clear'])
  .it('runs hello', ctx => {
    expect(ctx.stdout).to.contain('hello world')
  })

  test
  .stdout()
  .command(['transfers:clear', '--name', 'jeff'])
  .it('runs hello --name jeff', ctx => {
    expect(ctx.stdout).to.contain('hello jeff')
  })
})
