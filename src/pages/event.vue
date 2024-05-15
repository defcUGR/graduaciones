<script setup lang="ts">
import { IconUsers } from "@tabler/icons-vue";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Search } from "lucide-vue-next";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useRouter } from "vue-router";

const router = useRouter();
</script>

<template>
  <div class="flex flex-row">
    <div class="flex flex-1 flex-col gap-4 p-4 md:gap-8 md:p-8">
      <Card class="xl:col-span-2">
        <CardHeader class="flex flex-row items-center justify-between">
          <div class="grid gap-2">
            <CardTitle>Asistentes</CardTitle>
          </div>
          <div class="relative max-w-sm items-center">
            <Input
              id="search"
              type="text"
              placeholder="Search..."
              class="pl-10"
            />
            <span
              class="absolute start-0 inset-y-0 flex items-center justify-center px-2"
            >
              <Search class="size-6 text-muted-foreground" />
            </span>
          </div>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Customer</TableHead>
                <TableHead class="hidden xl:table-column"> Type </TableHead>
                <TableHead class="hidden xl:table-column"> Status </TableHead>
                <TableHead class="hidden xl:table-column"> Date </TableHead>
                <TableHead class="text-right"> Amount </TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow>
                <TableCell>
                  <div class="font-medium">Liam Johnson</div>
                  <div class="hidden text-sm text-muted-foreground md:inline">
                    liam@example.com
                  </div>
                </TableCell>
                <TableCell class="hidden xl:table-column"> Sale </TableCell>
                <TableCell class="hidden xl:table-column">
                  <Badge class="text-xs" variant="outline"> Approved </Badge>
                </TableCell>
                <TableCell
                  class="hidden md:table-cell lg:hidden xl:table-column"
                >
                  2023-06-23
                </TableCell>
                <TableCell class="text-right"> $250.00 </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <div class="font-medium">Olivia Smith</div>
                  <div class="hidden text-sm text-muted-foreground md:inline">
                    olivia@example.com
                  </div>
                </TableCell>
                <TableCell class="hidden xl:table-column"> Refund </TableCell>
                <TableCell class="hidden xl:table-column">
                  <Badge class="text-xs" variant="outline"> Declined </Badge>
                </TableCell>
                <TableCell
                  class="hidden md:table-cell lg:hidden xl:table-column"
                >
                  2023-06-24
                </TableCell>
                <TableCell class="text-right"> $150.00 </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <div class="font-medium">Noah Williams</div>
                  <div class="hidden text-sm text-muted-foreground md:inline">
                    noah@example.com
                  </div>
                </TableCell>
                <TableCell class="hidden xl:table-column">
                  Subscription
                </TableCell>
                <TableCell class="hidden xl:table-column">
                  <Badge class="text-xs" variant="outline"> Approved </Badge>
                </TableCell>
                <TableCell
                  class="hidden md:table-cell lg:hidden xl:table-column"
                >
                  2023-06-25
                </TableCell>
                <TableCell class="text-right"> $350.00 </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <div class="font-medium">Emma Brown</div>
                  <div class="hidden text-sm text-muted-foreground md:inline">
                    emma@example.com
                  </div>
                </TableCell>
                <TableCell class="hidden xl:table-column"> Sale </TableCell>
                <TableCell class="hidden xl:table-column">
                  <Badge class="text-xs" variant="outline"> Approved </Badge>
                </TableCell>
                <TableCell
                  class="hidden md:table-cell lg:hidden xl:table-column"
                >
                  2023-06-26
                </TableCell>
                <TableCell class="text-right"> $450.00 </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <div class="font-medium">Liam Johnson</div>
                  <div class="hidden text-sm text-muted-foreground md:inline">
                    liam@example.com
                  </div>
                </TableCell>
                <TableCell class="hidden xl:table-column"> Sale </TableCell>
                <TableCell class="hidden xl:table-column">
                  <Badge class="text-xs" variant="outline"> Approved </Badge>
                </TableCell>
                <TableCell
                  class="hidden md:table-cell lg:hidden xl:table-column"
                >
                  2023-06-27
                </TableCell>
                <TableCell class="text-right"> $550.00 </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
    <div class="flex flex-2 flex-col gap-4 p-4 md:gap-8 md:p-8">
      <Card>
        <CardHeader
          class="flex flex-row items-center justify-between space-y-0 pb-2"
        >
          <CardTitle class="text-sm font-medium"> Estadísticas </CardTitle>
          <IconUsers class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">10 / 143 personas</div>
          <p class="text-xs text-muted-foreground">
            13 % de entradas registradas
          </p>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Recent Sales</CardTitle>
        </CardHeader>
        <CardContent class="grid gap-8">
          <div class="flex items-center gap-4">
            <Avatar class="hidden h-9 w-9 sm:flex">
              <AvatarImage src="/avatars/01.png" alt="Avatar" />
              <AvatarFallback>OM</AvatarFallback>
            </Avatar>
            <div class="grid gap-1">
              <p class="text-sm font-medium leading-none">Olivia Martin</p>
              <p class="text-sm text-muted-foreground">
                olivia.martin@email.com
              </p>
            </div>
            <div class="ml-auto font-medium">+$1,999.00</div>
          </div>
          <div class="flex items-center gap-4">
            <Avatar class="hidden h-9 w-9 sm:flex">
              <AvatarImage src="/avatars/02.png" alt="Avatar" />
              <AvatarFallback>JL</AvatarFallback>
            </Avatar>
            <div class="grid gap-1">
              <p class="text-sm font-medium leading-none">Jackson Lee</p>
              <p class="text-sm text-muted-foreground">jackson.lee@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$39.00</div>
          </div>
          <div class="flex items-center gap-4">
            <Avatar class="hidden h-9 w-9 sm:flex">
              <AvatarImage src="/avatars/03.png" alt="Avatar" />
              <AvatarFallback>IN</AvatarFallback>
            </Avatar>
            <div class="grid gap-1">
              <p class="text-sm font-medium leading-none">Isabella Nguyen</p>
              <p class="text-sm text-muted-foreground">
                isabella.nguyen@email.com
              </p>
            </div>
            <div class="ml-auto font-medium">+$299.00</div>
          </div>
          <div class="flex items-center gap-4">
            <Avatar class="hidden h-9 w-9 sm:flex">
              <AvatarImage src="/avatars/04.png" alt="Avatar" />
              <AvatarFallback>WK</AvatarFallback>
            </Avatar>
            <div class="grid gap-1">
              <p class="text-sm font-medium leading-none">William Kim</p>
              <p class="text-sm text-muted-foreground">will@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$99.00</div>
          </div>
          <div class="flex items-center gap-4">
            <Avatar class="hidden h-9 w-9 sm:flex">
              <AvatarImage src="/avatars/05.png" alt="Avatar" />
              <AvatarFallback>SD</AvatarFallback>
            </Avatar>
            <div class="grid gap-1">
              <p class="text-sm font-medium leading-none">Sofia Davis</p>
              <p class="text-sm text-muted-foreground">sofia.davis@email.com</p>
            </div>
            <div class="ml-auto font-medium">+$39.00</div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
