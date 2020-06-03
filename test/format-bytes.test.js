const {expect, test} = require('@oclif/test')
const formatBytes = require('../src/format-bytes')

describe('FormatBytes', () => {
  test
  .it('Should return 3.60 MB', () => {
    expect(formatBytes(3777350)).to.equal('3.6 MB')
  })

  test
  .it('Should return 0 Bytes', () => {
    expect(formatBytes(0)).to.equal('0 Bytes')
  })
})
