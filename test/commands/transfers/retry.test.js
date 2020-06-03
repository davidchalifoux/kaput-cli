const {expect, test} = require('@oclif/test')

describe('transfers:retry', () => {
  test
  .stdout()
  .command(['transfers:retry'])
  .it('runs hello', ctx => {
    expect(ctx.stdout).to.contain('hello world')
  })

  test
  .stdout()
  .command(['transfers:retry', '--name', 'jeff'])
  .it('runs hello --name jeff', ctx => {
    expect(ctx.stdout).to.contain('hello jeff')
  })
})
