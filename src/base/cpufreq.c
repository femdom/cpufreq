/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 *
 *  Licensed under the terms of the GNU GPL License version 2.
 */


#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "cpufreq.h"
#include "sysfs.h"

#define BUFFER_SIZE 512

struct Cpu {
  long freq;
  long latency;
  long limits[2];
  long policy_min;
  long policy_max;
  char policy_governor[BUFFER_SIZE];
};


static struct Cpu all_cpus[] = {
  {2400000, 1000, {100000, 1000000}, 100000, 1000000, {"performance"}},
  {-EACCES, -EACCES, {-EACCES, -EACCES}, -EACCES, -EACCES, {"a"}}
};


inline unsigned long _process_result(long result) {
  if (result >= 0) {
    return result;
  } else {
    errno = -result;
    return result;
  }
}

int cpufreq_cpu_exists(unsigned int cpu) {
  return cpu <= 1 ? 0 : -1;
}

unsigned long cpufreq_get_freq_kernel(unsigned int cpu) {
  return _process_result(all_cpus[cpu].freq);
}

unsigned long cpufreq_get_freq_hardware(unsigned int cpu)
{
  if (geteuid() != 0) {
    errno = EACCES;
    return 0;
  }

  return cpufreq_get_freq_kernel(cpu);
}

unsigned long cpufreq_get_transition_latency(unsigned int cpu)
{
  return _process_result(all_cpus[cpu].latency);
}

int cpufreq_get_hardware_limits(unsigned int cpu,
				unsigned long *min,
				unsigned long *max)
{
  if ((!min) || (!max))
    return -EINVAL;

  long cpu_min = all_cpus[cpu].limits[0];
  long cpu_max = all_cpus[cpu].limits[1];

  if (cpu_min < 0) {
    errno = -cpu_min;
    return cpu_min;
  }

  if (cpu_max < 0) {
    errno = -cpu_max;
    return cpu_max;
  }

  *min = cpu_min;
  *max = cpu_max;

  return 0;
}

char *cpufreq_get_driver(unsigned int cpu)
{
	return sysfs_get_freq_driver(cpu);
}

void cpufreq_put_driver(char *ptr)
{
	if (!ptr)
		return;
	free(ptr);
}

struct cpufreq_policy *cpufreq_get_policy(unsigned int cpu) {
  long min = all_cpus[cpu].policy_min;
  long max = all_cpus[cpu].policy_max;

  if (min < 0) {
    errno = -max;
    return NULL;
  }

  if (max < 0) {
    errno = -max;
    return NULL;
  }

  struct cpufreq_policy* policy = malloc(sizeof(struct cpufreq_policy));

  if (!policy) {
    return NULL;
  }

  policy->governor = strndup(all_cpus[cpu].policy_governor, BUFFER_SIZE);
  if (!policy->governor) {
    free(policy);
    return NULL;
  }

  policy->min = min;
  policy->max = max;

  return policy;
}

void cpufreq_put_policy(struct cpufreq_policy *policy)
{
	if ((!policy) || (!policy->governor))
		return;

	free(policy->governor);
	policy->governor = NULL;
	free(policy);
}

struct cpufreq_available_governors *cpufreq_get_available_governors(unsigned
								int cpu)
{
	return sysfs_get_freq_available_governors(cpu);
}

void cpufreq_put_available_governors(struct cpufreq_available_governors *any)
{
	struct cpufreq_available_governors *tmp, *next;

	if (!any)
		return;

	tmp = any->first;
	while (tmp) {
		next = tmp->next;
		if (tmp->governor)
			free(tmp->governor);
		free(tmp);
		tmp = next;
	}
}


struct cpufreq_available_frequencies
*cpufreq_get_available_frequencies(unsigned int cpu)
{
	return sysfs_get_available_frequencies(cpu);
}

void cpufreq_put_available_frequencies(struct cpufreq_available_frequencies
				*any) {
	struct cpufreq_available_frequencies *tmp, *next;

	if (!any)
		return;

	tmp = any->first;
	while (tmp) {
		next = tmp->next;
		free(tmp);
		tmp = next;
	}
}


struct cpufreq_affected_cpus *cpufreq_get_affected_cpus(unsigned int cpu)
{
	return sysfs_get_freq_affected_cpus(cpu);
}

void cpufreq_put_affected_cpus(struct cpufreq_affected_cpus *any)
{
	struct cpufreq_affected_cpus *tmp, *next;

	if (!any)
		return;

	tmp = any->first;
	while (tmp) {
		next = tmp->next;
		free(tmp);
		tmp = next;
	}
}


struct cpufreq_affected_cpus *cpufreq_get_related_cpus(unsigned int cpu)
{
	return sysfs_get_freq_related_cpus(cpu);
}

void cpufreq_put_related_cpus(struct cpufreq_affected_cpus *any)
{
	cpufreq_put_affected_cpus(any);
}


int cpufreq_set_policy(unsigned int cpu, struct cpufreq_policy *policy)
{
	if (!policy || !(policy->governor))
		return -EINVAL;

    if (geteuid() != 0) {
      errno = EACCES;
      return -EACCES;
    }

	return sysfs_set_freq_policy(cpu, policy);
}


int cpufreq_modify_policy_min(unsigned int cpu, unsigned long min_freq)
{
	return sysfs_modify_freq_policy_min(cpu, min_freq);
}


int cpufreq_modify_policy_max(unsigned int cpu, unsigned long max_freq)
{
	return sysfs_modify_freq_policy_max(cpu, max_freq);
}


int cpufreq_modify_policy_governor(unsigned int cpu, char *governor)
{
	if ((!governor) || (strlen(governor) > 19))
		return -EINVAL;

	return sysfs_modify_freq_policy_governor(cpu, governor);
}

int cpufreq_set_frequency(unsigned int cpu, unsigned long target_frequency)
{
	return sysfs_set_frequency(cpu, target_frequency);
}

struct cpufreq_stats *cpufreq_get_stats(unsigned int cpu,
					unsigned long long *total_time)
{
	return sysfs_get_freq_stats(cpu, total_time);
}

void cpufreq_put_stats(struct cpufreq_stats *any)
{
	struct cpufreq_stats *tmp, *next;

	if (!any)
		return;

	tmp = any->first;
	while (tmp) {
		next = tmp->next;
		free(tmp);
		tmp = next;
	}
}

unsigned long cpufreq_get_transitions(unsigned int cpu)
{
	return sysfs_get_freq_transitions(cpu);
}
