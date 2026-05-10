# stui

A rust tui library to allow for html like decleration of gui

```xml
<Box hasborder="true">
    <Column gap="8">
        <Box hasborder="true">
            <Text text="test" length="8" height="4"/>
        </Box>
        <Row gap="8">
            <Box hasborder="true">
                <Input placeholder="placeholder" length="8" height="4"/>
            </Box>
            <Input placeholder="placeholder" length="8" height="4"/>
        </Row>
    </Column>
</Box>
```